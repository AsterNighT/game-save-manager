#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod backup;
mod archive;
mod errors;
mod ipc_handler;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            ipc_handler::local_config_check,
            ipc_handler::open_url,
            ipc_handler::choose_save_file,
            ipc_handler::choose_save_dir,
            ipc_handler::get_local_config,
            ipc_handler::add_game,
            ipc_handler::local_config_check,
            ipc_handler::apply_backup,
            ipc_handler::delete_backup,
            ipc_handler::delete_game,
            ipc_handler::get_backups_info,
            ipc_handler::set_config,
            ipc_handler::reset_settings,
            ipc_handler::backup_save,
            ipc_handler::open_backup_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
