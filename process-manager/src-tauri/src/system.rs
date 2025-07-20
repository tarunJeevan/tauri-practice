use std::thread::sleep;
use serde::{Deserialize, Serialize};
use sysinfo::{DiskRefreshKind, Disks, MemoryRefreshKind, System};

// Struct to contain system info
#[derive(Serialize, Deserialize)]
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
#[tauri::command]
pub fn get_sys_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_memory_specifics(
        MemoryRefreshKind::nothing().with_ram()
    );
    
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

/// Gets all disks on the system
///
/// Returns a vector of `DiskInfo` structs, one for each disk
#[tauri::command]
pub fn get_all_disks() -> Vec<DiskInfo> {
    let sys_disks = Disks::new_with_refreshed_list_specifics(
        DiskRefreshKind::nothing().with_storage()
    );
    
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
    
    #[test]
    fn test_system_info() {
        // TODO: Test get_sys_info() return
    }

    #[test]
    fn test_disks_info() {
        // TODO: Test get_all_disks() return
    }
}