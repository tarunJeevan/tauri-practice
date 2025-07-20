export interface ProcessInfo {
    id: string,
    name: string,
    owner: string,
    running_time_formatted: string,
    memory_in_bytes: string,
    status: string,
    cpu_usage_percent: number,
}