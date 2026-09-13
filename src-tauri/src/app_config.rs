use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub war_thunder_files: String,
    pub localization: bool,
    #[serde(default)]
    pub last_game_version: Option<String>,
}

pub fn get_app_root() -> Result<PathBuf, String> {
    // 1. Check current working directory and its parents for config.json
    if let Ok(cwd) = std::env::current_dir() {
        let mut check_dir = Some(cwd.as_path());
        while let Some(dir) = check_dir {
            if dir.join("config.json").exists() {
                return Ok(dir.to_path_buf());
            }
            check_dir = dir.parent();
        }
    }

    // 2. Check executable directory and its parents for config.json
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

        if let Some(exe_dir) = exe_path.parent() {
            return Ok(exe_dir.to_path_buf());
        }
    }

    Err("Failed to determine application root directory".to_string())
}

pub fn atomic_write<P: AsRef<std::path::Path>>(path: P, content: &[u8]) -> Result<(), String> {
    let path = path.as_ref();
    let tmp_path = path.with_extension("tmp_write");
    std::fs::write(&tmp_path, content).map_err(|e| format!("Failed to write temporary file: {}", e))?;
    std::fs::rename(&tmp_path, path).map_err(|e| format!("Failed to atomically replace file: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn load_app_config() -> Result<AppConfig, String> {
    let config_path = get_app_root()?.join("config.json");
    let content = std::fs::read_to_string(&config_path)
        .map_err(|err| format!("Failed to read config.json: {}", err))?;
    serde_json::from_str::<AppConfig>(&content)
        .map_err(|err| format!("Failed to parse config.json: {}", err))
}

pub fn save_app_config(config: &AppConfig) -> Result<(), String> {
    let config_path = get_app_root()?.join("config.json");
    let json_string = serde_json::to_string_pretty(config)
        .map_err(|err| err.to_string())?;

    atomic_write(&config_path, json_string.as_bytes())
}