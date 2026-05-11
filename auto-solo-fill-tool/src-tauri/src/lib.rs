use commands::{
    export_xlsx, read_data, write_data, read_text_file, open_file_dialog,
    get_llm_config, save_llm_config, test_connection,
    get_recent_sessions, ai_quality_check, process_analysis,
    generate_unsatisfied_reason,
    github_get_username, github_create_repo, github_create_branch,
};

mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            read_data, write_data,
            read_text_file, open_file_dialog,
            get_llm_config, save_llm_config, test_connection,
            get_recent_sessions,
            ai_quality_check, process_analysis, generate_unsatisfied_reason,
            github_get_username, github_create_repo, github_create_branch,
            export_xlsx,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
