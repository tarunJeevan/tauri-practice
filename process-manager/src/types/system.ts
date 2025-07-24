export interface SystemInfo {
    name: string,
    os: string,
    cpu_arch: string,
    cpu_usage_percent: number,
    total_memory: string,
    used_memory: string,
}

export interface DiskInfo {
    name: string,
    total_space: string,
    used_space: string,
}