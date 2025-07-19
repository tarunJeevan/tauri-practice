use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;
use sysinfo::{Pid, System, Users};

#[derive(Serialize, Deserialize)]
struct SystemInfo {
    name: String,         // NOTE: Use System::host_name()
    os: String,           // NOTE: Use System::distribution_id()
    cpu_arch: String,     // NOTE: Use System::cpu_arch()
    cpu_usage: f32,       // NOTE: Use System::global_cpu_usage()
    total_memory: String, // NOTE: Use System::total_memory()
    used_memory: String,  // NOTE: Use System::used_memory()
                          // TODO: Disk?
                          // TODO: GPU?
}

/// Gets system information such as Hostname, OS, CPU stats, RAM stats, etc.
///
/// Returns a `SystemInfo` struct containing system information
#[tauri::command]
fn get_sys_info() -> SystemInfo {
    let mut sys = System::new_all(); // TODO: Optimize with appropriate RefreshKind
    sys.refresh_all();

    SystemInfo {
        name: System::host_name().unwrap_or_else(|| "<Unknown>".to_string()),
        os: System::distribution_id(),
        cpu_arch: System::cpu_arch(),
        cpu_usage: sys.global_cpu_usage(),
        total_memory: sys.total_memory().to_string(),
        used_memory: sys.used_memory().to_string(),
    }
}

// Struct to contain individual process info
#[derive(Serialize, Deserialize)]
struct ProcessInfo {
    id: String,
    name: String,
    owner: String,
    running_time_formatted: String, // NOTE: Use run_time() but needs to be formatted
    memory_in_bytes: String,        // NOTE: Use memory()
    status: String,                 // NOTE: Use status()
    cpu_usage_percent: f32, // NOTE: Use Process::cpu_usage() and divide by System::cpus().len()
}

fn format_run_time(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 86400) / 60;
    let seconds = secs % 60;

    format!("{days:02} days {hours:02} hrs {minutes:02} mins {seconds:02} secs")
}

/// Gets all running processes on the system
///
/// Returns a vector of `ProcessInfo` structs, one for each process
#[tauri::command]
fn list_processes() -> Vec<ProcessInfo> {
    let mut sys = System::new_all(); // TODO: Optimize with appropriate RefreshKind
    let users = Users::new_with_refreshed_list();
    sys.refresh_all();

    sys.processes()
        .iter()
        .map(|(id, process)| ProcessInfo {
            id: id.to_string(),
            name: process.name().to_string_lossy().into_owned(),
            owner: match process.user_id() {
                Some(user_id) => users.get_user_by_id(user_id).unwrap().name().to_owned(),
                None => String::new(),
            },
            running_time_formatted: format_run_time(process.run_time()),
            memory_in_bytes: process.memory().to_string(),
            status: process.status().to_string(),
            cpu_usage_percent: process.cpu_usage() / sys.cpus().len() as f32,
        })
        .collect::<Vec<ProcessInfo>>()
}

/// Kills a running process with the given id
///
/// `id` is the Pid of the process to be terminated
///
/// Returns true if successfully terminated
#[tauri::command]
fn kill_process_by_id(id: &str) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();

    let pid = Pid::from_str(id).unwrap(); // FIXME: Handle errors more gracefully

    // `kill()` sends the kill signal and returns true if the signal was successfully sent
    sys.process(pid).unwrap().kill()
    // TODO: Rewrite to use `kill_and_wait()` which waits until the process terminates before returning with an exit code and set a timeout which stops the attempt after some time and returns with failure
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_sys_info,
            list_processes,
            kill_process_by_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
