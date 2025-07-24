use std::process::{Child, Command};
use std::str::FromStr;
use std::sync::Mutex;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, Signal, System, UpdateKind, Users};
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::time::{interval, sleep, timeout};

use crate::MonitorUpdateState;

// Struct to contain individual process info
#[derive(Serialize, Deserialize, Clone)]
pub struct ProcessInfo {
    id: String,
    name: String,
    owner: String,
    running_time_formatted: String,
    memory_used: String,
    status: String,
    cpu_usage_percent: f32,
}

/// Formats the process runtime into a readable string
///
/// `secs` is the process runtime in seconds
///
/// Returns a formatted String (e.g., "2 day(s) 1 hr(s) 42 min(s) 16 sec(s)")
fn format_run_time(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    format!("{days:02} day(s) {hours:02} hr(s) {minutes:02} min(s) {seconds:02} sec(s)")
}

/// Formats process memory into a readable string
///
/// `bytes` is the process memory usage in bytes
///
/// Returns a formatted String (e.g., "2 MB")
fn format_memory(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < units.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    // Format to 1 decimal place if size is not a whole number
    if size.fract() == 0.0 {
        format!("{size:.0} {}", units[unit])
    } else {
        format!("{size:.1} {}", units[unit])
    }
}

/// Gets an up-to-date list of processes on the system
///
/// Returns a vector of `ProcessInfo` structs, one for each process
fn get_current_processes() -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    let users = Users::new_with_refreshed_list();

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL); // Required for accurate CPU stats
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing()
            .with_cpu()
            .with_memory()
            .with_user(UpdateKind::Always),
    );

    let mut procs = sys
        .processes()
        .iter()
        .map(|(id, process)| ProcessInfo {
            id: id.to_string(),
            name: process.name().to_string_lossy().into_owned(),
            owner: match process.user_id() {
                Some(user_id) => users.get_user_by_id(user_id).unwrap().name().to_owned(),
                None => String::new(),
            },
            running_time_formatted: format_run_time(process.run_time()),
            memory_used: format_memory(process.memory()),
            status: process.status().to_string(),
            cpu_usage_percent: process.cpu_usage() / sys.cpus().len() as f32,
        })
        .collect::<Vec<ProcessInfo>>();

    // Sort by cpu usage by default. Frontend can implement further sorting functionality
    procs.sort_by(|a, b| {
        b.cpu_usage_percent
            .partial_cmp(&a.cpu_usage_percent)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    procs
}

/// Regularly updates frontend on all system processes
///
/// `app` is used to emit event to the frontend
#[tauri::command]
pub async fn monitor_processes(app: AppHandle) {
    // Poll for process updates every second
    let mut interval_timer = interval(Duration::from_millis(1000));

    tokio::spawn(async move {
        loop {
            // Check state and exit loop if the flag is set
            let stop_updates = {
                let state = app.state::<Mutex<MonitorUpdateState>>();
                let state_guard = state.lock().unwrap();
                state_guard.stop_process_updates
            };
            if stop_updates {
                println!("Stopping process updates");
                break;
            }
            interval_timer.tick().await;

            let procs = get_current_processes();
            // Emit the event globally and handle potential error
            if let Err(err) = app.emit("process_list_update", procs) {
                eprintln!("Failed to emit process_list_update event. Error: {err}");
            };
        }
    });
}

/// Updates the MonitorUpdateState to stop process updates
/// 
/// `state` is a reference to the MonitorUpdateState injected by Tauri
/// 
/// Returns a Unit Type (null in JavaScript) if successful and a String error if unsuccessful 
#[tauri::command]
pub fn stop_monitoring_processes(state: State<'_, Mutex<MonitorUpdateState>>) -> Result<(), String> {
    if let Ok(mut state_guard) = state.lock() {
        state_guard.stop_process_updates = true;
    } else {
        return Err("Failed to acquire lock on monitoring state".to_owned())
    };    
    Ok(())
}

/// Tries to kill a process gracefully using SIGTERM
///
/// `id` is the Pid of the process to be terminated
///
/// Returns a Result with a boolean value to indicate (un)successful signal dispatch or a String error
#[tauri::command]
pub async fn try_kill_process_by_id(id: &str) -> Result<bool, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Get Pid from id
    let pid = match Pid::from_str(id) {
        Ok(p) => p,
        Err(e) => return Err(format!("Invalid process ID ({id}) format: {e}")),
    };

    // Get the process from Pid
    let Some(process) = sys.process(pid) else {
        return Err(format!("Process with Pid {id} not found."));
    };

    // Attempt graceful termination
    if let Some(success) = process.kill_with(Signal::Term) {
        Ok(success)
    } else {
        Err(format!(
            "Graceful termination of process {id} failed. SIGTERM not supported."
        ))
    }
}

/// Forcefully kills a process using SIGKILL
///
/// `id` is the Pid of the process to be killed
///
/// Returns a Result with a Unit Value to indicate a successful termination or a String error
#[tauri::command]
pub async fn force_kill_process_by_id(id: &str) -> Result<(), String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Get Pid from id
    let pid = match Pid::from_str(id) {
        Ok(p) => p,
        Err(e) => return Err(format!("Invalid process ID ({id}) format: {e}")),
    };

    // Get the process from Pid
    let Some(process) = sys.process(pid) else {
        return Err(format!("Process with ID {id} not found."));
    };

    // Send KILL signal
    if !process.kill() {
        return Err(format!("Failed to send KILL signal to process {id}."));
    }

    // Timeout block: wait up to 3 secs for the process to disappear
    let wait_result = timeout(Duration::from_secs(3), async {
        let mut local_sys = System::new_all();

        loop {
            local_sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

            if sys.process(pid).is_none() {
                break;
            }
            sleep(Duration::from_millis(200)).await;
        }
    })
    .await;

    match wait_result {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Timed out waiting for process {id} to terminate.")),
    }

    // NOTE: Old code
    // // Create a task to wait for the process to terminate within timeout_duration
    // let wait_handle = tokio::task::spawn_blocking(move || {
    //     let mut local_sys = System::new_all();
    //
    //     let start_time = std::time::Instant::now();
    //     loop {
    //         // Refresh info for just this process
    //         local_sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
    //         if local_sys.process(pid).is_none() {
    //             // Process successfully terminated
    //             return Ok(());
    //         }
    //         if start_time.elapsed() >= timeout_duration {
    //             // Timeout reached
    //             return Err(format!(
    //                 "Process {pid} did not terminate within timeout period of {timeout_duration:?}."
    //             ));
    //         }
    //         // Small sleep to avoid busy-waiting
    //         std::thread::sleep(Duration::from_millis(50));
    //     }
    // });
    //
    // // Get the timeout result (Add small buffer to the timout for the overall task
    // let wait_result = timeout(Duration::from_secs(6), wait_handle).await;
    //
    // match wait_result {
    //     // The spawned task completed successfully and the process was terminated
    //     Ok(Ok(Ok(()))) => Ok(true),
    //     // Timeout reached
    //     Ok(Ok(Err(e))) => Err(format!("Process {id} failed with error: {e}")),
    //     // spawn_blocking task error
    //     Ok(Err(e)) => Err(format!(
    //         "An unexpected error occurred while waiting for process {id} to terminate: {e}"
    //     )),
    //     // Timeout error
    //     Err(_) => Err(format!(
    //         "Termination of process {id} timed out at a higher level (spawn_blocking task did not complete in time."
    //     )
    //     )
    // }
}

/// Spawns a mock process that simply sleeps for 30 seconds
///
/// Returns the `Child` handler for the created process
#[allow(dead_code)]
fn spawn_dummy_process() -> Child {
    Command::new("sleep")
        .arg("30")
        .spawn()
        .expect("Failed to spawn dummy process.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use sysinfo::ProcessStatus;
    use tokio::runtime::Runtime;

    #[test]
    fn test_format_run_time() {
        assert_eq!(
            format_run_time(0),
            String::from("00 day(s) 00 hr(s) 00 min(s) 00 sec(s)")
        );
        assert_eq!(
            format_run_time(3661),
            String::from("00 day(s) 01 hr(s) 01 min(s) 01 sec(s)")
        );
        assert_eq!(
            format_run_time(90061),
            String::from("01 day(s) 01 hr(s) 01 min(s) 01 sec(s)")
        );
    }

    #[test]
    fn test_format_memory() {
        assert_eq!(format_memory(0), String::from("0 B"));
        assert_eq!(format_memory(1023), String::from("1023 B"));

        assert_eq!(format_memory(1024), String::from("1 KB"));
        assert_eq!(format_memory(1234), String::from("1.2 KB"));

        assert_eq!(format_memory(1048576), String::from("1 MB"));

        assert_eq!(format_memory(1073741824), String::from("1 GB"));
    }

    #[test]
    fn test_list_processes() {
        let pattern = r"^\d{2} day\(s\) \d{2} hr\(s\) \d{2} min\(s\) \d{2} sec\(s\)$";
        let re = Regex::new(pattern).unwrap();

        let possible_status_list = vec![
            ProcessStatus::Dead.to_string(),
            ProcessStatus::Idle.to_string(),
            ProcessStatus::Run.to_string(),
            ProcessStatus::Parked.to_string(),
            ProcessStatus::LockBlocked.to_string(),
            ProcessStatus::Sleep.to_string(),
            ProcessStatus::Stop.to_string(),
            ProcessStatus::Tracing.to_string(),
            ProcessStatus::UninterruptibleDiskSleep.to_string(),
            ProcessStatus::Wakekill.to_string(),
            ProcessStatus::Waking.to_string(),
            ProcessStatus::Zombie.to_string(),
        ];

        let procs = get_current_processes();

        for proc in procs {
            assert!(!proc.id.is_empty());
            assert!(proc.id.parse::<u32>().is_ok());

            assert!(!proc.name.is_empty());

            assert!(!proc.running_time_formatted.is_empty());
            assert!(re.is_match(&proc.running_time_formatted));

            assert!(!proc.memory_used.is_empty());
            assert!(proc.memory_used.contains("B"));

            assert!(!proc.status.is_empty());
            assert!(possible_status_list.contains(&proc.status));

            assert!(proc.cpu_usage_percent >= 0.0);
        }
    }

    #[test]
    fn test_try_kill_process_by_id() {
        #[allow(clippy::zombie_processes)]
        let child = spawn_dummy_process();
        let child_id = child.id().to_string();

        let rt = Runtime::new().unwrap();

        // Test process ID verification
        let verification_result = rt.block_on(try_kill_process_by_id("invalid_process"));
        assert!(verification_result.is_err());
        assert!(verification_result
            .unwrap_err()
            .contains("Invalid process ID (invalid_process) format:"));

        let kill_result = rt.block_on(try_kill_process_by_id(&child_id));
        assert!(kill_result.is_ok());
    }

    #[test]
    fn test_force_kill_process_by_id() {
        #[allow(clippy::zombie_processes)]
        let child = spawn_dummy_process();
        let child_id = child.id().to_string();

        let rt = Runtime::new().unwrap();

        // Test process ID verification
        let verification_result = rt.block_on(try_kill_process_by_id("invalid_process"));
        assert!(verification_result.is_err());
        assert!(verification_result
            .unwrap_err()
            .contains("Invalid process ID (invalid_process) format:"));

        let kill_result = rt.block_on(try_kill_process_by_id(&child_id));
        assert!(kill_result.is_ok());
    }
}
