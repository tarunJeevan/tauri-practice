export interface ProcessInfo {
    id: string,
    name: string,
    owner: string,
    running_time_formatted: string,
    memory_used: string,
    status: string,
    cpu_usage_percent: number,
}