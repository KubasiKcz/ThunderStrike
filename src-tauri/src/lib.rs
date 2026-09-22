mod app_config;
mod localization;

use app_config::{load_app_config, save_war_thunder_path};
use localization::{
    acknowledge_game_version,
    apply_all_diffs_to_game,
    apply_diff_to_csv,
    check_game_update,
    check_lang_folder_exists,
    fetch_clean_lang_from_datamine,
    get_current_game_version,
    get_localization_file,
    get_localization_files,
    get_starred_entries,
    get_starred_keys,
    reset_lang_folder,
    restore_original_csv,
    save_localization_diff,
    save_starred_keys,
    set_localization,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_app_config,
            save_war_thunder_path,
            set_localization,
            check_lang_folder_exists,
            get_localization_files,
            get_localization_file,
            save_localization_diff,
            get_starred_keys,
            save_starred_keys,
            get_starred_entries,
            get_current_game_version,
            check_game_update,
            acknowledge_game_version,
            reset_lang_folder,
            apply_diff_to_csv,
            apply_all_diffs_to_game,
            restore_original_csv,
            fetch_clean_lang_from_datamine
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}