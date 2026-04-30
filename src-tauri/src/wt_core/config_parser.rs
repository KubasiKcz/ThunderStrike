use encoding_rs::UTF_16LE;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSettings {
    #[serde(default)]
    pub lang: String,
    #[serde(rename = "gameLocation", default)]
    pub game_location: String,
    #[serde(rename = "gameLang", default)]
    pub game_lang: String,
    #[serde(rename = "lastWtVersion", default)]
    pub last_wt_version: String,
    #[serde(rename = "firstLaunch")]
    pub first_launch: Vec<bool>,
    #[serde(rename = "wtUsername", default)]
    pub wt_nickname: String,
    #[serde(rename = "steamUsername", default)]
    pub steam_username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    #[serde(rename = "userSettings")]
    pub user_settings: UserSettings,
}

#[tauri::command]
pub fn save_setting(key: String, value: String) -> Result<(), String> {
    let mut path = std::path::PathBuf::from("settings.json");
    if !path.exists() {
        path = std::path::PathBuf::from("../settings.json");
    }

    let path_str = path.to_str().unwrap_or("settings.json");

    let data =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read {}: {}", path_str, e))?;
    let mut settings: Settings =
        serde_json::from_str(&data).map_err(|e| format!("Invalid JSON: {}", e))?;

    match key.as_str() {
        "lang" => settings.user_settings.lang = value,
        "gameLocation" => settings.user_settings.game_location = value,
        "gameLang" => settings.user_settings.game_lang = value,
        "lastWtVersion" => settings.user_settings.last_wt_version = value,
        "wtUsername" => settings.user_settings.wt_nickname = value,
        "firstLaunch" => {
            if value == "false" {
                settings.user_settings.first_launch = vec![false];
            } else {
                settings.user_settings.first_launch = vec![true];
            }
        }
        "steamUsername" => settings.user_settings.steam_username = value,
        _ => return Err("Invalid key".to_string()),
    }

    let new_data = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize: {}", e))?;
    fs::write(&path, new_data).map_err(|e| format!("Failed to write {}: {}", path_str, e))?;
    Ok(())
}

#[tauri::command]
pub fn get_setting(key: String) -> Result<String, String> {
    let mut path = std::path::PathBuf::from("settings.json");
    if !path.exists() {
        path = std::path::PathBuf::from("../settings.json");
    }

    if !path.exists() {
        return Err("Settings file not found".to_string());
    }

    let data = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let settings: Settings = serde_json::from_str(&data).map_err(|e| e.to_string())?;

    match key.as_str() {
        "lang" => Ok(settings.user_settings.lang),
        "gameLocation" => Ok(settings.user_settings.game_location),
        "gameLang" => Ok(settings.user_settings.game_lang),
        "lastWtVersion" => Ok(settings.user_settings.last_wt_version),
        "wtUsername" => Ok(settings.user_settings.wt_nickname),
        "steamUsername" => Ok(settings.user_settings.steam_username),
        "firstLaunch" => Ok(format!("{:?}", settings.user_settings.first_launch)),
        _ => Err("Invalid key".to_string()),
    }
}

#[tauri::command]
pub fn save_custom_edits(content: String) -> Result<(), String> {
    let mut path = std::path::PathBuf::from("custom_edits.json");
    if !path.exists() && std::path::PathBuf::from("../custom_edits.json").exists() {
        path = std::path::PathBuf::from("../custom_edits.json");
    }

    fs::write(&path, content).map_err(|e| format!("Failed to save custom_edits.json: {}", e))
}

#[tauri::command]
pub fn get_custom_edits() -> Result<String, String> {
    let mut path = std::path::PathBuf::from("custom_edits.json");
    if !path.exists() && std::path::PathBuf::from("../custom_edits.json").exists() {
        path = std::path::PathBuf::from("../custom_edits.json");
    }

    if !path.exists() {
        return Ok("{}".to_string());
    }

    fs::read_to_string(&path).map_err(|e| format!("Failed to read custom_edits.json: {}", e))
}

#[derive(Serialize)]
pub struct WTPathResponse {
    status: &'static str,
    path: Option<String>,
    languages: Option<Vec<String>>,
    msg: Option<String>,
}

#[tauri::command]
pub fn find_wt_path(user_input: String) -> WTPathResponse {
    if user_input.trim().is_empty() {
        return WTPathResponse {
            status: "error",
            path: None,
            languages: None,
            msg: Some("Path is missing.".to_string()),
        };
    }

    let clean_path = user_input.replace("\"", "").trim().to_string();
    let path = std::path::Path::new(&clean_path);

    let mut final_path = None;
    let mut game_root = None;

    if clean_path.ends_with("lang") && path.exists() {
        final_path = Some(path.to_path_buf());
    } else {
        let config_blk = path.join("config.blk");
        if config_blk.exists() {
            game_root = Some(path.to_path_buf());
            let potential_lang = path.join("lang");
            if potential_lang.exists() {
                final_path = Some(potential_lang);
            }
        }
    }

    if final_path.is_none() && game_root.is_some() {
        return WTPathResponse {
            status: "waiting_for_lang",
            path: Some(game_root.unwrap().to_string_lossy().to_string()),
            languages: None,
            msg: Some("Game found, but 'lang' folder is missing. Please start and then close War Thunder to generate it.".to_string()),
        };
    }

    if let Some(f_path) = final_path {
        let mut languages = Vec::new();
        let filenames = ["menu.csv", "main.csv"];
        for fname in filenames {
            let target_csv = f_path.join(fname);
            if target_csv.exists() {
                if let Ok(content) = read_file_with_encoding(&target_csv) {
                    if let Some(first_line) = content.lines().next() {
                        let separator = if first_line.contains(';') { ';' } else { ',' };
                        let parts = first_line.split(separator);
                        for part in parts.skip(1) {
                            let mut lang_str = part.trim().replace("\"", "");
                            if let Some(idx) = lang_str.find("<maxchars:") {
                                lang_str = lang_str[..idx].to_string();
                            }
                            lang_str = lang_str
                                .replace("<", "")
                                .replace(">", "")
                                .trim()
                                .to_string();
                            let lower = lang_str.to_lowercase();
                            if !lang_str.is_empty()
                                && lower != "maxchars"
                                && lower != "max_chars"
                                && lower != "id"
                            {
                                languages.push(lang_str);
                            }
                        }
                    }
                    if !languages.is_empty() {
                        break;
                    }
                }
            }
        }

        languages.sort();
        languages.dedup();

        return WTPathResponse {
            status: "ok",
            path: Some(f_path.to_string_lossy().to_string()),
            languages: Some(languages),
            msg: None,
        };
    }

    WTPathResponse {
        status: "error",
        path: None,
        languages: None,
        msg: Some(
            "Invalid path. Make sure you selected the War Thunder installation folder.".to_string(),
        ),
    }
}

fn read_file_with_encoding(path: &std::path::Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;

    if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xFE {
        let (res, _, has_errors) = UTF_16LE.decode(&bytes[2..]);
        if has_errors {
            return Err("Encoding error in UTF-16 LE file".to_string());
        }
        return Ok(res.to_string());
    }

    match String::from_utf8(bytes) {
        Ok(s) => Ok(s),
        Err(e) => {
            let temp_bytes = e.into_bytes();
            let (res, _, has_errors) = UTF_16LE.decode(&temp_bytes);
            if !has_errors {
                Ok(res.to_string())
            } else {
                Err("File is not UTF-8 and forced UTF-16 LE decoding failed.".to_string())
            }
        }
    }
}

#[tauri::command]
pub fn list_csv_files(path: String) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    let entries = std::fs::read_dir(&path).map_err(|e| e.to_string())?;
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_file() {
            if let Some(ext) = p.extension() {
                if ext == "csv" {
                    if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                        files.push(name.to_string());
                    }
                }
            }
        }
    }
    files.sort();
    Ok(files)
}

#[tauri::command]
pub fn read_text_file(path: String) -> Result<String, String> {
    read_file_with_encoding(std::path::Path::new(&path))
}

#[tauri::command]
pub fn write_text_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| format!("Failed to write {}: {}", path, e))
}

#[tauri::command]
pub fn delete_file(path: String) -> Result<(), String> {
    let p = std::path::Path::new(&path);
    if p.exists() {
        std::fs::remove_file(p).map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn copy_file(src: String, dest: String) -> Result<(), String> {
    std::fs::copy(src, dest)
        .map_err(|e| e.to_string())
        .map(|_| ())
}

#[tauri::command]
pub fn get_wt_version(wt_path: String) -> Result<String, String> {
    let parent = std::path::Path::new(&wt_path)
        .parent()
        .unwrap_or(std::path::Path::new(&wt_path));
    let ver_path = parent.join("pkg_main.version");
    if ver_path.exists() {
        std::fs::read_to_string(ver_path).map_err(|e| e.to_string())
    } else {
        Err("pkg_main.version not found".to_string())
    }
}

#[tauri::command]
pub fn reset_settings() -> Result<(), String> {
    let mut path = std::path::PathBuf::from("settings.json");
    if !path.exists() {
        path = std::path::PathBuf::from("../settings.json");
    }

    let default_settings = r#"{
    "userSettings": {
        "lang": "en",
        "gameLocation": "",
        "gameLang": "English",
        "lastWtVersion": "",
        "firstLaunch": [true],
        "wtUsername": "",
        "steamUsername": ""
    }
}"#;

    fs::write(&path, default_settings).map_err(|e| format!("Failed to reset settings: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn toggle_mod_granular(game_path: String, mod_type: String, enabled: bool) -> Result<(), String> {
    let mut game_root = std::path::PathBuf::from(&game_path);
    if game_root.ends_with("lang") {
        if let Some(parent) = game_root.parent() {
            game_root = parent.to_path_buf();
        }
    }

    let blk_path = game_root.join("config.blk");
    if !blk_path.exists() {
        return Err("config.blk not found in game directory".to_string());
    }

    let mut content = fs::read_to_string(&blk_path).map_err(|e| e.to_string())?;
    let val_b = if enabled { "yes" } else { "no" };

    match mod_type.as_str() {
        "localization" => {
            if content.contains("testLocalization:b=") {
                content = content.replace(
                    "testLocalization:b=yes",
                    &format!("testLocalization:b={}", val_b),
                );
                content = content.replace(
                    "testLocalization:b=no",
                    &format!("testLocalization:b={}", val_b),
                );
            } else if let Some(pos) = content.find("debug{") {
                content.insert_str(pos + 6, &format!("\n  testLocalization:b={}\n", val_b));
            } else {
                content.push_str(&format!("\ndebug{{\n  testLocalization:b={}\n}}\n", val_b));
            }
        }
        "sound" => {
            if content.contains("sound{") {
                if content.contains("enable_mod:b=") {
                    content =
                        content.replace("enable_mod:b=yes", &format!("enable_mod:b={}", val_b));
                    content =
                        content.replace("enable_mod:b=no", &format!("enable_mod:b={}", val_b));
                } else {
                    if let Some(pos) = content.find("sound{") {
                        content.insert_str(pos + 6, &format!("\n  enable_mod:b={}\n", val_b));
                    }
                }
                if content.contains("fmod_sound_enable:b=") {
                    content = content.replace(
                        "fmod_sound_enable:b=yes",
                        &format!("fmod_sound_enable:b={}", val_b),
                    );
                    content = content.replace(
                        "fmod_sound_enable:b=no",
                        &format!("fmod_sound_enable:b={}", val_b),
                    );
                } else {
                    if let Some(pos) = content.find("sound{") {
                        content
                            .insert_str(pos + 6, &format!("\n  fmod_sound_enable:b={}\n", val_b));
                    }
                }
            } else {
                content.push_str(&format!(
                    "\nsound{{\n  enable_mod:b={}\n  fmod_sound_enable:b={}\n}}\n",
                    val_b, val_b
                ));
            }
        }
        "hangar" => {
            if content.contains("hangarBlk:t=") {
                if enabled {
                } else {
                    let lines: Vec<String> = content
                        .lines()
                        .filter(|l| !l.trim().starts_with("hangarBlk:t="))
                        .map(|l| l.to_string())
                        .collect();
                    content = lines.join("\n");
                }
            } else if enabled {
                content.push_str("\nhangarBlk:t=\"my_hangar.blk\"\n");
            }
        }
        _ => return Err("Unknown mod type".to_string()),
    }

    fs::write(blk_path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_app_path() -> String {
    let mut path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    if !path.join("settings.json").exists() && std::path::PathBuf::from("../settings.json").exists()
    {
        path = std::path::PathBuf::from("..");
    }
    path.canonicalize()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| path.to_string_lossy().to_string())
}
