use std::collections::HashMap;
use std::path::PathBuf;
use crate::app_config::{atomic_write, get_app_root, load_app_config, save_app_config};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct LocalizationEntry {
    pub key: String,
    pub original: String,
    pub custom: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct LocalizationFileData {
    pub entries: Vec<LocalizationEntry>,
    pub languages: Vec<String>,
    pub selected_lang: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct VersionCheckResult {
    pub is_updated: bool,
    pub current_version: String,
    pub previous_version: Option<String>,
}

pub fn sanitize_csv_name(file_name: &str) -> Result<String, String> {
    let path = std::path::Path::new(file_name);
    if path.parent() != Some(std::path::Path::new(""))
        || file_name.contains("..")
        || file_name.contains('/')
        || file_name.contains('\\')
    {
        return Err("Invalid file name: path traversal characters detected".to_string());
    }

    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Invalid file name".to_string())?;

    let trimmed = stem.trim();
    if trimmed.is_empty() {
        return Err("File name cannot be empty".to_string());
    }

    Ok(format!("{}.csv", trimmed))
}

fn get_game_directory() -> Result<PathBuf, String> {
    let root = get_app_root()?;
    let config = load_app_config()?;
    let wt_base_path = PathBuf::from(&config.war_thunder_files);
    if wt_base_path.is_relative() {
        Ok(root.join(wt_base_path))
    } else {
        Ok(wt_base_path)
    }
}

fn get_diff_path(file_name: &str) -> Result<PathBuf, String> {
    let sanitized = sanitize_csv_name(file_name)?;
    let clean_stem = sanitized.trim_end_matches(".csv");
    let root = get_app_root()?;
    Ok(root
        .join("mods")
        .join("localization")
        .join("diffs")
        .join(format!("{}.json", clean_stem)))
}

fn load_diff(file_name: &str) -> HashMap<String, String> {
    let diff_path = match get_diff_path(file_name) {
        Ok(path) => path,
        Err(_) => return HashMap::new(),
    };

    if !diff_path.exists() {
        return HashMap::new();
    }

    let file_result = std::fs::read_to_string(&diff_path);
    let content = match file_result {
        Ok(c) => c,
        Err(_) => return HashMap::new(),
    };

    serde_json::from_str::<HashMap<String, String>>(&content).unwrap_or_default()
}

fn get_backup_path(file_name: &str) -> Result<PathBuf, String> {
    let sanitized = sanitize_csv_name(file_name)?;
    let root = get_app_root()?;
    Ok(root
        .join("mods")
        .join("localization")
        .join("backups")
        .join("lang_original")
        .join(&sanitized))
}

pub fn ensure_backup_exists(file_name: &str) -> Result<(), String> {
    let sanitized = sanitize_csv_name(file_name)?;
    let backup_path = get_backup_path(&sanitized)?;
    if !backup_path.exists() {
        let game_dir = get_game_directory()?;
        let source_path = game_dir.join("lang").join(&sanitized);
        if source_path.exists() {
            if let Some(parent) = backup_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|err| format!("Failed to create backup directory: {}", err))?;
            }
            std::fs::copy(&source_path, &backup_path)
                .map_err(|err| format!("Failed to copy backup file: {}", err))?;
        }
    }
    Ok(())
}

fn clean_header(header: &str) -> String {
    header
        .trim()
        .trim_matches('"')
        .trim_matches('<')
        .trim_matches('>')
        .trim()
        .to_string()
}

fn clean_cell(cell: &str) -> String {
    let trimmed = cell.trim();
    if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}

#[tauri::command]
pub fn check_lang_folder_exists() -> Result<bool, String> {
    if let Ok(game_dir) = get_game_directory() {
        let lang_dir = game_dir.join("lang");
        if lang_dir.exists() && lang_dir.is_dir() {
            return Ok(true);
        }
    }
    // Also check if local backups have any CSV files (e.g. from datamine)
    if let Ok(root) = get_app_root() {
        let backup_dir = root.join("mods").join("localization").join("backups").join("lang_original");
        if backup_dir.exists() && backup_dir.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&backup_dir) {
                if entries.flatten().any(|e| {
                    e.path().extension().map(|ext| ext.eq_ignore_ascii_case("csv")).unwrap_or(false)
                }) {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}

#[tauri::command]
pub fn get_localization_files() -> Result<Vec<String>, String> {
    let mut file_set = std::collections::BTreeSet::new();

    if let Ok(game_dir) = get_game_directory() {
        let lang_dir = game_dir.join("lang");
        if lang_dir.exists() && lang_dir.is_dir() {
            if let Ok(dir_entries) = std::fs::read_dir(&lang_dir) {
                for entry in dir_entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            if ext.eq_ignore_ascii_case("csv") {
                                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                                    file_set.insert(file_name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if let Ok(root) = get_app_root() {
        let backup_dir = root.join("mods").join("localization").join("backups").join("lang_original");
        if backup_dir.exists() && backup_dir.is_dir() {
            if let Ok(dir_entries) = std::fs::read_dir(&backup_dir) {
                for entry in dir_entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            if ext.eq_ignore_ascii_case("csv") {
                                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                                    file_set.insert(file_name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if file_set.is_empty() {
        return Err("LANG_DIR_NOT_FOUND".to_string());
    }

    let mut files: Vec<String> = file_set.into_iter().collect();
    files.sort();
    Ok(files)
}

#[tauri::command]
pub fn get_localization_file(
    file_name: String,
    target_lang: Option<String>,
) -> Result<LocalizationFileData, String> {
    let sanitized_name = sanitize_csv_name(&file_name)?;

    let backup_path = get_backup_path(&sanitized_name)?;
    let csv_path = if backup_path.exists() {
        backup_path
    } else {
        let game_dir = get_game_directory()?;
        let lang_dir = game_dir.join("lang");
        if !lang_dir.exists() || !lang_dir.is_dir() {
            return Err("LANG_DIR_NOT_FOUND".to_string());
        }
        lang_dir.join(&sanitized_name)
    };

    let csv_content = std::fs::read_to_string(&csv_path)
        .map_err(|_| format!("CSV_FILE_NOT_FOUND: {}", sanitized_name))?;

    let user_diff = load_diff(&sanitized_name);

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .flexible(true)
        .quoting(true)
        .from_reader(csv_content.as_bytes());

    let headers = match reader.headers() {
        Ok(h) => h.clone(),
        Err(_) => {
            return Ok(LocalizationFileData {
                entries: Vec::new(),
                languages: Vec::new(),
                selected_lang: "English".to_string(),
            })
        }
    };

    let mut available_languages: Vec<String> = Vec::new();
    let mut lang_columns: Vec<(String, usize)> = Vec::new();

    for (idx, raw_header) in headers.iter().enumerate() {
        let cleaned = clean_header(raw_header);
        if cleaned.is_empty() {
            continue;
        }
        let lower = cleaned.to_lowercase();
        if lower.starts_with("id") || lower == "comments" || lower == "max_chars" {
            continue;
        }
        available_languages.push(cleaned.clone());
        lang_columns.push((cleaned, idx));
    }

    let search_lang = target_lang
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .and_then(|lang| available_languages.iter().find(|l| l.eq_ignore_ascii_case(lang)))
        .or_else(|| available_languages.iter().find(|l| l.eq_ignore_ascii_case("English")))
        .or_else(|| available_languages.first())
        .cloned()
        .unwrap_or_else(|| "English".to_string());

    let mut lang_col_index = 1;
    for (lang_name, col_idx) in &lang_columns {
        if lang_name.eq_ignore_ascii_case(&search_lang) {
            lang_col_index = *col_idx;
            break;
        }
    }

    let mut entries: Vec<LocalizationEntry> = Vec::new();
    let mut seen_keys = std::collections::HashSet::new();

    for result in reader.records() {
        let record = match result {
            Ok(r) => r,
            Err(_) => continue,
        };

        if record.is_empty() {
            continue;
        }

        let raw_key = record.get(0).unwrap_or("");
        let key = clean_cell(raw_key);
        if key.is_empty() || key.starts_with("//") {
            continue;
        }

        if seen_keys.contains(&key) {
            continue;
        }
        seen_keys.insert(key.clone());

        let original_text = if record.len() > lang_col_index {
            clean_cell(record.get(lang_col_index).unwrap_or(""))
        } else {
            String::new()
        };

        let custom_text = user_diff.get(&key).cloned();

        entries.push(LocalizationEntry {
            key,
            original: original_text,
            custom: custom_text,
        });
    }

    Ok(LocalizationFileData {
        entries,
        languages: available_languages,
        selected_lang: search_lang,
    })
}

#[tauri::command]
pub fn save_localization_diff(
    file_name: String,
    diffs: HashMap<String, String>,
) -> Result<(), String> {
    let sanitized_name = sanitize_csv_name(&file_name)?;
    let clean_stem = sanitized_name.trim_end_matches(".csv");

    if !diffs.is_empty() && clean_stem != "starred" {
        let _ = ensure_backup_exists(&sanitized_name);
    }

    let diff_path = get_diff_path(&sanitized_name)?;
    if let Some(parent) = diff_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|err| format!("Failed to create diff directory: {}", err))?;
    }

    // Empty-string values mean the user intentionally cleared the field,
    // but patching game files with empty text is pointless – skip them.
    let filtered: HashMap<String, String> = diffs
        .into_iter()
        .filter(|(_, v)| !v.is_empty())
        .collect();

    let json_string = serde_json::to_string_pretty(&filtered)
        .map_err(|err| err.to_string())?;

    atomic_write(&diff_path, json_string.as_bytes())
}

const DEFAULT_STARRED_TEMPLATE: &str = include_str!("default_starred.txt");

fn parse_starred_keys(content: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with("//")
            || trimmed.starts_with('#')
            || trimmed.starts_with('<')
        {
            continue;
        }
        let key = trimmed.to_string();
        if !seen.contains(&key) {
            seen.insert(key.clone());
            keys.push(key);
        }
    }
    keys
}

fn get_starred_file_path() -> Result<PathBuf, String> {
    let root = get_app_root()?;
    Ok(root.join("mods").join("localization").join("starred.txt"))
}

#[tauri::command]
pub fn get_starred_keys() -> Result<Vec<String>, String> {
    let starred_path = get_starred_file_path()?;

    if starred_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&starred_path) {
            let keys = parse_starred_keys(&content);
            if !keys.is_empty() {
                return Ok(keys);
            }
        }
    }

    if let Some(parent) = starred_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = atomic_write(&starred_path, DEFAULT_STARRED_TEMPLATE.as_bytes());

    Ok(parse_starred_keys(DEFAULT_STARRED_TEMPLATE))
}

#[tauri::command]
pub fn save_starred_keys(keys: Vec<String>) -> Result<(), String> {
    let starred_path = get_starred_file_path()?;
    if let Some(parent) = starred_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let mut lines = vec!["<ID|readonly|noverify>".to_string()];
    lines.extend(keys);
    let content = lines.join("\n") + "\n";

    atomic_write(&starred_path, content.as_bytes())
}

#[tauri::command]
pub fn get_starred_entries(target_lang: Option<String>) -> Result<LocalizationFileData, String> {
    let keys = get_starred_keys()?;
    let key_set: std::collections::HashSet<&str> = keys.iter().map(|k| k.as_str()).collect();

    let menu_data = get_localization_file("menu.csv".to_string(), target_lang.clone())?;
    let mut entries_map: HashMap<String, LocalizationEntry> = HashMap::new();

    for entry in menu_data.entries {
        if key_set.contains(entry.key.as_str()) {
            entries_map.insert(entry.key.clone(), entry);
        }
    }

    let missing_keys: Vec<&String> = keys.iter().filter(|k| !entries_map.contains_key(*k)).collect();
    if !missing_keys.is_empty() {
        if let Ok(files) = get_localization_files() {
            for file in files {
                if file.eq_ignore_ascii_case("menu.csv") {
                    continue;
                }
                if let Ok(other_data) = get_localization_file(file, target_lang.clone()) {
                    for entry in other_data.entries {
                        if key_set.contains(entry.key.as_str()) && !entries_map.contains_key(&entry.key) {
                            entries_map.insert(entry.key.clone(), entry);
                        }
                    }
                }
                if keys.iter().all(|k| entries_map.contains_key(k)) {
                    break;
                }
            }
        }
    }

    let starred_diff = load_diff("starred");

    let mut result_entries: Vec<LocalizationEntry> = Vec::new();
    for key in keys {
        let custom_override = starred_diff.get(&key).cloned();
        if let Some(mut existing) = entries_map.remove(&key) {
            if custom_override.is_some() {
                existing.custom = custom_override;
            }
            result_entries.push(existing);
        } else {
            result_entries.push(LocalizationEntry {
                key: key.clone(),
                original: String::new(),
                custom: custom_override,
            });
        }
    }

    Ok(LocalizationFileData {
        entries: result_entries,
        languages: menu_data.languages,
        selected_lang: menu_data.selected_lang,
    })
}

pub fn update_blk_localization(content: &str, enabled: bool) -> String {
    let target_value = if enabled { "yes" } else { "no" };
    let target_line = format!("  testLocalization:b={}", target_value);

    let mut result_lines: Vec<String> = Vec::new();
    let mut in_debug_block = false;
    let mut found_debug_block = false;
    let mut found_localization_key = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("debug{") || trimmed.starts_with("debug {") {
            in_debug_block = true;
            found_debug_block = true;
            result_lines.push(line.to_string());
            continue;
        }

        if in_debug_block {
            if trimmed.starts_with("testLocalization:b=") {
                result_lines.push(target_line.clone());
                found_localization_key = true;
                continue;
            }

            if trimmed == "}" {
                if !found_localization_key {
                    result_lines.push(target_line.clone());
                    found_localization_key = true;
                }
                in_debug_block = false;
                result_lines.push(line.to_string());
                continue;
            }
        }

        result_lines.push(line.to_string());
    }

    if !found_debug_block {
        result_lines.push(String::new());
        result_lines.push("debug {".to_string());
        result_lines.push(target_line);
        result_lines.push("}".to_string());
    }

    result_lines.join("\n")
}

#[tauri::command]
pub fn set_localization(enabled: bool) -> Result<(), String> {
    let mut config = load_app_config()?;
    config.localization = enabled;
    save_app_config(&config)?;

    let game_dir = get_game_directory()?;
    let blk_path = game_dir.join("config.blk");
    let blk_content = std::fs::read_to_string(&blk_path)
        .map_err(|err| format!("Failed to read config.blk: {}", err))?;

    let updated_blk_content = update_blk_localization(&blk_content, enabled);
    atomic_write(&blk_path, updated_blk_content.as_bytes())
        .map_err(|err| format!("Failed to write config.blk: {}", err))
}

#[tauri::command]
pub fn get_current_game_version() -> Result<String, String> {
    let game_dir = get_game_directory()?;
    let version_path = game_dir.join("version");
    if !version_path.exists() {
        return Err("Game version file not found in game directory".to_string());
    }
    let content = std::fs::read_to_string(&version_path)
        .map_err(|e| format!("Failed to read game version file: {}", e))?;
    Ok(content.trim().to_string())
}

#[tauri::command]
pub fn check_game_update() -> Result<VersionCheckResult, String> {
    let current_version = match get_current_game_version() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let config = load_app_config()?;
    let previous_version = config.last_game_version.clone();

    let is_updated = match &previous_version {
        Some(prev) => prev != &current_version,
        None => false,
    };

    Ok(VersionCheckResult {
        is_updated,
        current_version,
        previous_version,
    })
}

#[tauri::command]
pub fn acknowledge_game_version(version: String) -> Result<(), String> {
    let mut config = load_app_config()?;
    config.last_game_version = Some(version);
    save_app_config(&config)
}

#[tauri::command]
pub fn reset_lang_folder() -> Result<(), String> {
    let game_dir = get_game_directory()?;
    let lang_dir = game_dir.join("lang");
    if lang_dir.exists() {
        std::fs::remove_dir_all(&lang_dir)
            .map_err(|e| format!("Failed to delete lang directory: {}", e))?;
    }

    let root = get_app_root()?;
    let backup_dir = root
        .join("mods")
        .join("localization")
        .join("backups")
        .join("lang_original");
    if backup_dir.exists() {
        let _ = std::fs::remove_dir_all(&backup_dir);
    }

    Ok(())
}

#[tauri::command]
pub fn apply_diff_to_csv(file_name: String, target_lang: Option<String>) -> Result<(), String> {
    let sanitized_name = sanitize_csv_name(&file_name)?;
    let clean_stem = sanitized_name.trim_end_matches(".csv");

    if clean_stem == "starred" {
        apply_all_diffs_to_game(target_lang)?;
        return Ok(());
    }

    let game_dir = get_game_directory()?;
    let lang_dir = game_dir.join("lang");
    if !lang_dir.exists() {
        let _ = std::fs::create_dir_all(&lang_dir);
    }
    let target_csv_path = lang_dir.join(&sanitized_name);
    let backup_path = get_backup_path(&sanitized_name)?;

    if !target_csv_path.exists() && !backup_path.exists() {
        return Err(format!("CSV file not found in game directory or backups: {}", sanitized_name));
    }

    let _ = ensure_backup_exists(&sanitized_name);

    let source_path = if backup_path.exists() {
        backup_path
    } else {
        target_csv_path.clone()
    };

    let csv_content = std::fs::read_to_string(&source_path)
        .map_err(|e| format!("Failed to read source CSV {}: {}", sanitized_name, e))?;

    let mut diff_map = load_diff(&sanitized_name);
    let starred_diff = load_diff("starred");
    for (k, v) in starred_diff {
        diff_map.entry(k).or_insert(v);
    }

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .flexible(true)
        .quoting(true)
        .from_reader(csv_content.as_bytes());

    let headers = reader
        .headers()
        .map_err(|e| format!("Failed to read CSV headers: {}", e))?
        .clone();

    let search_lang = target_lang
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .and_then(|lang| headers.iter().find(|h| clean_header(h).eq_ignore_ascii_case(lang)))
        .or_else(|| headers.iter().find(|h| clean_header(h).eq_ignore_ascii_case("English")))
        .map(|h| clean_header(h))
        .unwrap_or_else(|| "English".to_string());

    let mut target_col_index = 1;
    for (idx, raw_header) in headers.iter().enumerate() {
        let cleaned = clean_header(raw_header);
        if cleaned.eq_ignore_ascii_case(&search_lang) {
            target_col_index = idx;
            break;
        }
    }

    let mut writer = csv::WriterBuilder::new()
        .delimiter(b';')
        .terminator(csv::Terminator::CRLF)
        .quote_style(csv::QuoteStyle::Necessary)
        .from_writer(Vec::new());

    writer.write_record(&headers).map_err(|e| e.to_string())?;

    for result in reader.records() {
        let record = result.map_err(|e| e.to_string())?;
        if record.is_empty() {
            writer.write_record(&record).map_err(|e| e.to_string())?;
            continue;
        }

        let raw_key = record.get(0).unwrap_or("");
        let key = clean_cell(raw_key);

        if let Some(custom_value) = diff_map.get(&key) {
            let mut modified_fields: Vec<String> = Vec::with_capacity(record.len());
            for (i, field) in record.iter().enumerate() {
                if i == target_col_index {
                    modified_fields.push(custom_value.clone());
                } else {
                    modified_fields.push(field.to_string());
                }
            }
            writer.write_record(&modified_fields).map_err(|e| e.to_string())?;
        } else {
            writer.write_record(&record).map_err(|e| e.to_string())?;
        }
    }

    writer.flush().map_err(|e| e.to_string())?;
    let output_bytes = writer.into_inner().map_err(|e| e.to_string())?;

    atomic_write(&target_csv_path, &output_bytes)
        .map_err(|e| format!("Failed to write patched CSV {}: {}", sanitized_name, e))?;

    Ok(())
}

#[tauri::command]
pub fn apply_all_diffs_to_game(target_lang: Option<String>) -> Result<usize, String> {
    let files = get_localization_files()?;
    let mut patched_count = 0;

    for file in files {
        if file.eq_ignore_ascii_case("starred.csv") {
            continue;
        }
        apply_diff_to_csv(file, target_lang.clone())?;
        patched_count += 1;
    }

    Ok(patched_count)
}

#[tauri::command]
pub fn restore_original_csv(file_name: String) -> Result<(), String> {
    let sanitized_name = sanitize_csv_name(&file_name)?;
    let clean_stem = sanitized_name.trim_end_matches(".csv");

    if clean_stem == "starred" {
        let starred_diff_path = get_diff_path("starred")?;
        if starred_diff_path.exists() {
            let _ = std::fs::remove_file(&starred_diff_path);
        }
        let _ = apply_all_diffs_to_game(None);
        return Ok(());
    }

    let game_dir = get_game_directory()?;
    let lang_dir = game_dir.join("lang");
    let target_csv_path = lang_dir.join(&sanitized_name);

    let backup_path = get_backup_path(&sanitized_name)?;
    if backup_path.exists() {
        if !lang_dir.exists() {
            let _ = std::fs::create_dir_all(&lang_dir);
        }
        let backup_bytes = std::fs::read(&backup_path)
            .map_err(|e| format!("Failed to read vanilla backup CSV: {}", e))?;
        atomic_write(&target_csv_path, &backup_bytes)
            .map_err(|e| format!("Failed to restore vanilla CSV: {}", e))?;
    }

    let diff_path = get_diff_path(&sanitized_name)?;
    if diff_path.exists() {
        let _ = std::fs::remove_file(&diff_path);
    }

    Ok(())
}

/// Downloads clean (vanilla) CSV files from the gszabi99/War-Thunder-Datamine repository
/// and saves them into the local backup folder (`mods/localization/backups/lang_original/`).
/// Returns the number of successfully downloaded files.
#[tauri::command]
pub fn fetch_clean_lang_from_datamine(file_names: Vec<String>) -> Result<u32, String> {
    let root = get_app_root()?;
    let backup_dir = root
        .join("mods")
        .join("localization")
        .join("backups")
        .join("lang_original");

    std::fs::create_dir_all(&backup_dir)
        .map_err(|e| format!("Failed to create backup directory: {}", e))?;

    let base_url = "https://raw.githubusercontent.com/gszabi99/War-Thunder-Datamine/master/lang.vromfs.bin_u/lang";

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let mut count = 0u32;
    let mut errors: Vec<String> = Vec::new();

    for raw_name in &file_names {
        let sanitized = match sanitize_csv_name(raw_name) {
            Ok(s) => s,
            Err(e) => {
                errors.push(format!("{}: {}", raw_name, e));
                continue;
            }
        };

        let url = format!("{}/{}", base_url, sanitized);
        match client.get(&url).send() {
            Ok(resp) if resp.status().is_success() => {
                match resp.bytes() {
                    Ok(bytes) => {
                        let out_path = backup_dir.join(&sanitized);
                        if let Err(e) = std::fs::write(&out_path, &bytes) {
                            errors.push(format!("{}: write failed – {}", sanitized, e));
                        } else {
                            count += 1;
                        }
                    }
                    Err(e) => errors.push(format!("{}: read failed – {}", sanitized, e)),
                }
            }
            Ok(resp) => {
                errors.push(format!("{}: HTTP {} – file may not exist in datamine", sanitized, resp.status()));
            }
            Err(e) => errors.push(format!("{}: request failed – {}", sanitized, e)),
        }
    }

    if count == 0 && !errors.is_empty() {
        return Err(errors.join("; "));
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_csv_name() {
        assert_eq!(sanitize_csv_name("menu.csv").unwrap(), "menu.csv");
        assert_eq!(sanitize_csv_name("menu").unwrap(), "menu.csv");
        assert!(sanitize_csv_name("../config.blk").is_err());
        assert!(sanitize_csv_name("sub/menu.csv").is_err());
        assert!(sanitize_csv_name("..\\menu.csv").is_err());
        assert!(sanitize_csv_name("").is_err());
    }

    #[test]
    fn test_clean_header_and_clean_cell() {
        assert_eq!(clean_header("<English>"), "English");
        assert_eq!(clean_header("\"<ID|readonly|noverify>\""), "ID|readonly|noverify");
        assert_eq!(clean_cell("\"hello world\""), "hello world");
        assert_eq!(clean_cell("plain_text"), "plain_text");
    }

    #[test]
    fn test_update_blk_localization() {
        let empty_blk = "graphics {\n  mode:t=\"fullscreen\"\n}";
        let updated_yes = update_blk_localization(empty_blk, true);
        assert!(updated_yes.contains("testLocalization:b=yes"));

        let updated_no = update_blk_localization(&updated_yes, false);
        assert!(updated_no.contains("testLocalization:b=no"));
        assert!(!updated_no.contains("testLocalization:b=yes"));
    }

    #[test]
    fn test_parse_starred_keys() {
        let content = "# Header comment\n<ID|readonly>\nmainmenu/btnExit\n\n// comment\nhud_target_destroyed\nmainmenu/btnExit";
        let keys = parse_starred_keys(content);
        assert_eq!(keys, vec!["mainmenu/btnExit", "hud_target_destroyed"]);
    }

    #[test]
    fn test_delta_patcher_csv_logic() {
        let csv_data = "<ID|readonly|noverify>;<English>;<Czech>\nmainmenu/btnExit;Exit;Konec\nmainmenu/btnOptions;Options;Možnosti\n";

        let mut diffs: HashMap<String, String> = HashMap::new();
        diffs.insert("mainmenu/btnExit".to_string(), "Quit Game".to_string());

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(true)
            .flexible(true)
            .quoting(true)
            .from_reader(csv_data.as_bytes());

        let headers = reader.headers().unwrap().clone();
        assert_eq!(clean_header(&headers[1]), "English");
        assert_eq!(clean_header(&headers[2]), "Czech");

        let mut writer = csv::WriterBuilder::new()
            .delimiter(b';')
            .terminator(csv::Terminator::CRLF)
            .quote_style(csv::QuoteStyle::Necessary)
            .from_writer(Vec::new());

        writer.write_record(&headers).unwrap();

        for result in reader.records() {
            let record = result.unwrap();
            let key = clean_cell(&record[0]);

            if let Some(custom) = diffs.get(&key) {
                let mut modified = Vec::new();
                for (i, field) in record.iter().enumerate() {
                    if i == 1 {
                        modified.push(custom.clone());
                    } else {
                        modified.push(field.to_string());
                    }
                }
                writer.write_record(&modified).unwrap();
            } else {
                writer.write_record(&record).unwrap();
            }
        }

        writer.flush().unwrap();
        let patched_output = String::from_utf8(writer.into_inner().unwrap()).unwrap();

        assert!(patched_output.contains("Quit Game"));
        assert!(patched_output.contains("Konec"));
        assert!(patched_output.contains("mainmenu/btnOptions;Options;Možnosti"));
    }
}