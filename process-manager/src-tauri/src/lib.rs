mod process;
mod system;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            system::get_sys_info,
            system::get_all_disks,
            process::list_processes,
            process::try_kill_process_by_id,
            process::force_kill_process_by_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
