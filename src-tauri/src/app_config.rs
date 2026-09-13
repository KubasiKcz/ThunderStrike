use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub war_thunder_files: String,
    pub localization: bool,
    #[serde(default)]
    pub last_game_version: Option<String>,
}

pub fn get_app_root() -> Result<PathBuf, String> {
    // 1. If running as an AppImage on Linux, use the directory containing the .AppImage file
    if let Ok(appimage_path) = std::env::var("APPIMAGE") {
        let path = PathBuf::from(appimage_path);
        if let Some(parent) = path.parent() {
            return Ok(parent.to_path_buf());
        }
    }

    // 2. Check current working directory and its parents for config.json (e.g. dev or portable folder)
    if let Ok(cwd) = std::env::current_dir() {
        let mut check_dir = Some(cwd.as_path());
        while let Some(dir) = check_dir {
            if dir.join("config.json").exists() {
                return Ok(dir.to_path_buf());
            }
            check_dir = dir.parent();
        }
    }

    // 3. Check executable directory and its parents for config.json (e.g. installed location on Windows)
    if let Ok(exe_path) = std::env::current_exe() {
        let mut check_dir = exe_path.parent();
        while let Some(dir) = check_dir {
            if dir.join("config.json").exists() {
                return Ok(dir.to_path_buf());
            }
            check_dir = dir.parent();
        }

        // Fallback in debug mode if config.json hasn't been created yet
        if cfg!(debug_assertions) {
            if let Some(exe_dir) = exe_path.parent() {
                let dev_root = exe_dir.join("../../../");
                if let Ok(clean) = std::fs::canonicalize(&dev_root) {
                    return Ok(clean);
                }
            }
        }

        // If not in a temporary mount (/tmp/.mount_*) or system read-only (/usr/bin) directory, use exe dir
        if let Some(exe_dir) = exe_path.parent() {
            let str_path = exe_dir.to_string_lossy();
            if !str_path.contains(".mount_") && !str_path.starts_with("/usr") && !str_path.starts_with("/bin") {
                return Ok(exe_dir.to_path_buf());
            }
        }
    }

    // 4. Standard system user configuration directory for installed packages (e.g. /usr/bin/thunderstrike from .deb / pacman)
    #[cfg(target_os = "windows")]
    let base_dir = std::env::var("APPDATA")
        .map(PathBuf::from)
        .or_else(|_| std::env::var("USERPROFILE").map(|p| PathBuf::from(p).join("AppData").join("Roaming")));

    #[cfg(target_os = "macos")]
    let base_dir = std::env::var("HOME")
        .map(|p| PathBuf::from(p).join("Library").join("Application Support"));

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let base_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| std::env::var("HOME").map(|p| PathBuf::from(p).join(".config")));

    if let Ok(base) = base_dir {
        let app_dir = base.join("thunderstrike");
        return Ok(app_dir);
    }

    Err("Failed to determine application root directory".to_string())
}

pub fn auto_detect_war_thunder_path() -> String {
    #[cfg(target_os = "windows")]
    let candidate_paths = [
        r"C:\Program Files (x86)\Steam\steamapps\common\War Thunder",
        r"C:\Steam\steamapps\common\War Thunder",
        r"D:\Steam\steamapps\common\War Thunder",
        r"D:\SteamLibrary\steamapps\common\War Thunder",
        r"E:\SteamLibrary\steamapps\common\War Thunder",
        r"C:\WarThunder",
    ];

    #[cfg(target_os = "macos")]
    let candidate_paths = [
        "Library/Application Support/Steam/steamapps/common/War Thunder",
    ];

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let candidate_paths = [
        ".local/share/Steam/steamapps/common/War Thunder",
        ".steam/steam/steamapps/common/War Thunder",
        ".var/app/com.valvesoftware.Steam/data/Steam/steamapps/common/War Thunder",
    ];

    #[cfg(not(target_os = "windows"))]
    if let Ok(home) = std::env::var("HOME") {
        let home_path = PathBuf::from(home);
        for rel in candidate_paths {
            let p = home_path.join(rel);
            if p.exists() && (p.join("config.blk").exists() || p.join("package.blk").exists()) {
                return p.to_string_lossy().to_string();
            }
        }
    }

    #[cfg(target_os = "windows")]
    for p in candidate_paths {
        let path = PathBuf::from(p);
        if path.exists() && (path.join("config.blk").exists() || path.join("package.blk").exists()) {
            return path.to_string_lossy().to_string();
        }
    }

    String::new()
}

pub fn ensure_app_initialized() -> Result<PathBuf, String> {
    let root = get_app_root()?;

    // Create root directory if needed
    if !root.exists() {
        std::fs::create_dir_all(&root)
            .map_err(|e| format!("Failed to create application root directory at {:?}: {}", root, e))?;
    }

    // Ensure mods/localization directories exist
    let diffs_dir = root.join("mods").join("localization").join("diffs");
    let backups_dir = root.join("mods").join("localization").join("backups").join("lang_original");
    if !diffs_dir.exists() {
        let _ = std::fs::create_dir_all(&diffs_dir);
    }
    if !backups_dir.exists() {
        let _ = std::fs::create_dir_all(&backups_dir);
    }

    // Ensure starred.txt exists with defaults if not present
    let starred_file = root.join("mods").join("localization").join("starred.txt");
    if !starred_file.exists() {
        let default_starred = include_str!("default_starred.txt");
        if let Some(parent) = starred_file.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&starred_file, default_starred);
    }

    // Ensure config.json exists
    let config_path = root.join("config.json");
    if !config_path.exists() {
        let wt_path = auto_detect_war_thunder_path();
        let default_config = AppConfig {
            war_thunder_files: wt_path,
            localization: false,
            last_game_version: None,
        };
        let content = serde_json::to_string_pretty(&default_config)
            .map_err(|e| e.to_string())?;
        atomic_write(&config_path, content.as_bytes())?;
    }

    Ok(root)
}

pub fn atomic_write<P: AsRef<std::path::Path>>(path: P, content: &[u8]) -> Result<(), String> {
    let path = path.as_ref();
    let tmp_path = path.with_extension("tmp_write");
    std::fs::write(&tmp_path, content).map_err(|e| format!("Failed to write temporary file: {}", e))?;
    #[cfg(target_os = "windows")]
    if path.exists() {
        let _ = std::fs::remove_file(path);
    }
    std::fs::rename(&tmp_path, path).map_err(|e| format!("Failed to atomically replace file: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn load_app_config() -> Result<AppConfig, String> {
    let root = ensure_app_initialized()?;
    let config_path = root.join("config.json");
    let content = std::fs::read_to_string(&config_path)
        .map_err(|err| format!("Failed to read config.json: {}", err))?;
    serde_json::from_str::<AppConfig>(&content)
        .map_err(|err| format!("Failed to parse config.json: {}", err))
}

pub fn save_app_config(config: &AppConfig) -> Result<(), String> {
    let root = ensure_app_initialized()?;
    let config_path = root.join("config.json");
    let json_string = serde_json::to_string_pretty(config)
        .map_err(|err| err.to_string())?;

    atomic_write(&config_path, json_string.as_bytes())
}