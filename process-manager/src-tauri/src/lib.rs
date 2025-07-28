use std::sync::Mutex;
use tauri::Manager;

mod process;
mod system;

#[derive(Default)]
struct MonitorUpdateState {
    stop_process_updates: bool,
    stop_system_updates: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            system::monitor_sys_info,
            system::stop_monitoring_system,
            system::get_all_disks,
            process::monitor_processes,
            process::stop_monitoring_processes,
            process::try_kill_process_by_id,
            process::force_kill_process_by_id,
        ])
        .setup(|app| {
            // Set default MonitorUpdateState
            app.manage(Mutex::new(MonitorUpdateState::default()));
            // // Create tokio runtime
            // let rt = tokio::runtime::Runtime::new().unwrap();
            // // Gett app handle clones
            // let process_app_handle = app.handle().clone();
            // let system_app_handle = app.handle().clone();
            //
            // // Start monitoring processes when the app starts up
            // rt.spawn(async move {
            //     monitor_processes(process_app_handle).await;
            // });
            // // Start monitoring system resource usage when the app starts up
            // rt.spawn(async move {
            //     monitor_sys_info(system_app_handle).await;
            // });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
