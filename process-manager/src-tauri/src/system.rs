use crate::MonitorUpdateState;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{DiskRefreshKind, Disks, MemoryRefreshKind, System};
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::time::interval;

// Struct to contain system info
#[derive(Serialize, Deserialize, Clone)]
pub struct SystemInfo {
    name: String,
    os: String,
    cpu_arch: String,
    cpu_usage_percent: f32,
    total_memory: String,
    used_memory: String,
    // TODO: GPU?
}

// Struct to contain disk info
#[derive(Serialize, Deserialize)]
pub struct DiskInfo {
    name: String,
    total_space: String,
    used_space: String,
}

/// Formats given number of bytes into a readable String
///
/// `bytes` is the number of bytes for memory or disk usage
///
/// Returns a formatted String (e.g., "2 MB")
fn format_bytes(bytes: u64) -> String {
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

/// Gets system information such as Hostname, OS, CPU stats, RAM stats, etc.
///
/// Returns a `SystemInfo` struct containing system information
fn get_sys_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());

    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL); // Required for accurate CPU usage stats
    sys.refresh_cpu_usage();

    SystemInfo {
        name: System::host_name().unwrap_or("<Unknown>".to_owned()),
        os: System::distribution_id(),
        cpu_arch: System::cpu_arch(),
        cpu_usage_percent: sys.global_cpu_usage(),
        total_memory: format_bytes(sys.total_memory()),
        used_memory: format_bytes(sys.used_memory()),
    }
}

/// Regularly updates frontend on system resource usage
///
/// `app` is used to emit event to the frontend
#[tauri::command]
pub async fn monitor_sys_info(app: AppHandle) {
    // Poll for system update every second
    let mut interval_timer = interval(Duration::from_millis(1000));

    tokio::spawn(async move {
        loop {
            // Check state and exit loop if the flag is set
            let stop_updates = {
                let state = app.state::<Mutex<MonitorUpdateState>>();
                let state_guard = state.lock().unwrap();
                state_guard.stop_system_updates
            };
            if stop_updates {
                println!("Stopping system updates");
                break;
            }
            interval_timer.tick().await;

            let sys_info = get_sys_info();
            // Emit the event globally and handle potential error
            if let Err(err) = app.emit("system_update", sys_info) {
                eprintln!("Failed to emit system_update event. Error: {err}");
            };
        }
    });
}

#[tauri::command]
pub fn stop_monitoring_system(state: State<'_, Mutex<MonitorUpdateState>>) -> Result<(), String> {
    if let Ok(mut state_guard) = state.lock() {
        state_guard.stop_system_updates = true;
    } else {
        return Err("Failed to acquire lock on monitoring state".to_owned());
    };
    Ok(())
}

/// Gets all disks on the system
///
/// Returns a vector of `DiskInfo` structs, one for each disk
#[tauri::command]
pub fn get_all_disks() -> Vec<DiskInfo> {
    let sys_disks =
        Disks::new_with_refreshed_list_specifics(DiskRefreshKind::nothing().with_storage());

    sys_disks
        .iter()
        .map(|disk| DiskInfo {
            name: disk.name().to_string_lossy().into_owned(),
            total_space: format_bytes(disk.total_space()),
            used_space: format_bytes(disk.total_space() - disk.available_space()),
        })
        .collect::<Vec<DiskInfo>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(123), "123 B");

        assert_eq!(format_bytes(1024), "1 KB");
        assert_eq!(format_bytes(1234), "1.2 KB");

        assert_eq!(format_bytes(1048576), "1 MB");
        assert_eq!(format_bytes(1234567), "1.2 MB");
    }

    #[test]
    fn test_get_system_info() {
        let info = get_sys_info();

        assert!(!info.name.is_empty());
        assert_eq!(
            info.name,
            env::var("HOSTNAME").unwrap_or("<Unknown>".to_owned())
        );

        assert!(!info.os.is_empty());
        // assert_eq!(info.os, String::from(env::consts::OS)); // NOTE: Getting info from /etc/os-release

        assert!(!info.cpu_arch.is_empty());
        assert_eq!(info.cpu_arch, String::from(env::consts::ARCH));

        assert!(info.cpu_usage_percent >= 0.0);

        assert!(!info.total_memory.is_empty());
        assert!(info.total_memory.contains("B"));

        assert!(!info.used_memory.is_empty());
        assert!(info.used_memory.contains("B"));
    }

    #[test]
    fn test_get_all_disks() {
        let disks = get_all_disks();

        for disk in disks {
            assert!(!disk.name.is_empty());

            assert!(!disk.total_space.is_empty());
            assert!(disk.total_space.contains("B"));

            assert!(!disk.used_space.is_empty());
            assert!(disk.used_space.contains("B"));
        }
    }
}
