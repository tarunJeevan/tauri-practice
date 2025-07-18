use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;
use sysinfo::{Pid, System};

/// Get OS name
///
/// Returns a static string containing the OS name (e.g., "linux", "macos", "windows", etc.)
#[tauri::command]
fn get_os_name() -> &'static str {
    env::consts::OS
}

// Struct to contain individual process info
#[derive(Serialize, Deserialize, Debug)]
struct ProcessInfo {
    id: String,
    name: String,
}

/// Gets all running processes on the system
///
/// Returns a vector of `ProcessInfo` structs, one for each process
#[tauri::command]
fn list_processes() -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.processes()
        .iter()
        .map(|(id, process)| ProcessInfo {
            id: id.to_string(),
            name: process.name().to_string_lossy().into_owned(),
        })
        .collect::<Vec<ProcessInfo>>()
}

#[derive(Serialize, Deserialize, Debug)]
struct ProcessDetails {
    id: String,
    // TODO: Add other process details such as CPU usage, memory usage, etc.
}

// TODO: Return detailed process properties based on sysinfo functions
// #[tauri::command]
// fn list_process_details(id: &str) -> ProcessDetails {}

/// Kills running process with the given id
///
/// `id` is the Pid of the process to be terminated
///
/// Returns true if successfully terminated
#[tauri::command]
fn kill_process_by_id(id: &str) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();

    let pid = Pid::from_str(id).unwrap(); // NOTE: Handle errors more gracefully

    // `kill()` sends the kill signal and returns true if the signal was successfully sent
    sys.process(pid).unwrap().kill()
    // NOTE: Rewrite to use `kill_and_wait()` which waits until the process terminates before returning with an exit code and set a timeout which stops the attempt after some time and returns with failure
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_os_name,
            list_processes,
            kill_process_by_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
