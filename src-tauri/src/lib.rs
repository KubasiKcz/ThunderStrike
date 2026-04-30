mod wt_core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_drpc::init())
        .invoke_handler(tauri::generate_handler![
            wt_core::config_parser::save_setting,
            wt_core::config_parser::get_setting,
            wt_core::config_parser::save_custom_edits,
            wt_core::config_parser::get_custom_edits,
            wt_core::config_parser::find_wt_path,
            wt_core::config_parser::list_csv_files,
            wt_core::config_parser::read_text_file,
            wt_core::config_parser::write_text_file,
            wt_core::config_parser::delete_file,
            wt_core::config_parser::copy_file,
            wt_core::config_parser::get_wt_version,
            wt_core::config_parser::reset_settings,
            wt_core::api_client::fetch_wt_profile,
            wt_core::api_client::scrape_wt_profile,
            wt_core::config_parser::toggle_mod_granular,
            wt_core::config_parser::get_app_path,
            wt_core::api_client::launch_wt
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
