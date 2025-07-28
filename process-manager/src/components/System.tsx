import {DiskInfo, SystemInfo} from "../types/system.ts";

interface ProgressRingProps {
    radius: number,
    stroke: number,
    progress: number,
    color: string,
    backgroundColor: string,
    label: string,
}

interface ProgressBarProps {
    total: string,
    completed: string,
    progress: number,
    color: string,
    backgroundColor: string,
}

// Helper function to parse memory strings into bytes
function memoryStringToBytes(memString: string): number {
    const parts = memString.split(" ");

    // Get value and unit from parts
    const value = parseFloat(parts[0]);
    const unit = parts[1];

    // Return memory in bytes based on unit for accurate percentages
    switch (unit) {
        case "B":
            return value;
        case "KB":
            return value * Math.pow(1024, 1);
        case "MB":
            return value * Math.pow(1024, 2);
        case "GB":
            return value * Math.pow(1024, 3);
        case "TB":
            return value * Math.pow(1024, 4);
        case "PB":
            return value * Math.pow(1024, 5);
        default:
            return 0;
    }
}

// Progress ring component for displaying system CPU and RAM usage
function ProgressRing({radius, stroke, progress, color, backgroundColor, label}: ProgressRingProps) {
    const normalizedRadius = radius - stroke * 2;
    const circumference = normalizedRadius * 2 * Math.PI;
    const strokeDashOffset = circumference - (progress / 100) * circumference;

    return (
        <svg
            height={radius * 2}
            width={radius * 2}
            viewBox={`0 0 ${radius * 2} ${radius * 2}`}
            className="transform -rotate-90" // Rotate to start progress from top
        >
            {/* Background circle */}
            <circle
                stroke={backgroundColor}
                fill="transparent" // Only the stroke should be visible to create ring effect
                strokeWidth={stroke}
                r={normalizedRadius}
                cx={radius}
                cy={radius}
            />
            {/* Progress circle */}
            <circle
                stroke={color}
                fill="transparent" // Only the stroke should be visible to create ring effect
                strokeWidth={stroke}
                strokeDasharray={circumference + ' ' + circumference} // Creates a solid line
                style={{strokeDashoffset: strokeDashOffset}} // Controls the fill level
                r={normalizedRadius}
                cx={radius}
                cy={radius}
                strokeLinecap="round" // Makes the ends of the stroke rounded
            />
            {/* Text label */}
            <text
                x="50%"
                y="50%"
                textAnchor="middle"
                dominantBaseline="middle"
                className="text-sm font-bold"
                fill={color}
                style={{transform: 'rotate(90)', transformOrigin: 'center'}} // Counter-rotate text
            >
                {label}
            </text>
        </svg>
    );
}

// Progress bar component for displaying disk space usage
function ProgressBar({total, completed, progress, color, backgroundColor}: ProgressBarProps) {
    // Ensure that progress is within a range of 0-100
    const clampedProgress = Math.max(0, Math.min(100, progress));

    // FIXME: Determine why progress bar isn't showing
    return (
        <div className={`w-1/2 h-6 rounded-full ${backgroundColor}`}>
            <div
                className={`h-full rounded-full ${color}`}
                style={{width: `${clampedProgress}%`}}
            >
                <span className="w-full text-center text-white text-xs">{completed} / {total}</span>
            </div>
        </div>
    );
}

export default function System({sysInfo, disks}: { sysInfo: SystemInfo, disks: DiskInfo[] }) {
    // Calculate memory usage percentage
    const totalMemoryInBytes = memoryStringToBytes(sysInfo.total_memory);
    const usedMemoryInBytes = memoryStringToBytes(sysInfo.used_memory);
    const memoryUsagePercent = totalMemoryInBytes > 0
        ? usedMemoryInBytes / totalMemoryInBytes * 100
        : 0;

    return (
        <div className="min-w-full flex flex-col p-4 gap-8">
            {/* System */}
            <div className="flex flex-col gap-2 p-6">
                <h2 className="text-xl font-bold text-gray-700">System Overview</h2>

                {/* Basic System Info */}
                <div className="flex flex-col gap-2 items-center mt-4">
                    <span className="font-semibold text-lg">Hostname: {sysInfo.name}</span>
                    <span className="font-semibold text-lg">OS: {sysInfo.os}</span>
                    <span className="font-semibold text-lg">CPU Architecture: {sysInfo.cpu_arch}</span>
                </div>

                {/* Display CPU and Memory Usage */}
                <div
                    className="mt-4 flex flex-col md:flex-row justify-around items-center space-y-8 md:space-y-0 md:space-x-8">
                    {/* CPU Usage Ring */}
                    <div className="flex flex-col items-center">
                        <p className="mb-2 text-gray-300 font-semibold">CPU Usage</p>
                        <ProgressRing
                            radius={60}
                            stroke={8}
                            progress={sysInfo.cpu_usage_percent}
                            color="#34D399" // Green
                            backgroundColor="#4B5563" // Gray
                            label={`${sysInfo.cpu_usage_percent.toFixed(0)}%`}
                        />
                    </div>

                    {/* Memory Usage Ring */}
                    <div className="flex flex-col items-center">
                        <p className="mb-2 text-gray-300 font-semibold">Memory Usage</p>
                        <ProgressRing
                            radius={60}
                            stroke={8}
                            progress={memoryUsagePercent}
                            color="#18A8EC" // Blue
                            backgroundColor="#4B5563" // Gray
                            label={`${memoryUsagePercent.toFixed(0)}%`}
                        />
                    </div>
                </div>
            </div>
            {/* Disks */}
            <div className="flex flex-col gap-2 p-6">
                {disks.map((disk, index) => {
                    // Calculate storage usage percent for all disks
                    const totalStorageInBytes = memoryStringToBytes(disk.total_space);
                    const usedStorageInBytes = memoryStringToBytes(disk.used_space);
                    const storageUsagePercent = totalStorageInBytes > 0
                        ? usedStorageInBytes / totalStorageInBytes * 100
                        : 0;

                    return (
                        <div key={index} className="flex flex-row gap-2">
                            <span className="text-sm font-semibold">Disk: {disk.name}</span>
                            <ProgressBar
                                total={disk.total_space}
                                completed={disk.used_space}
                                progress={storageUsagePercent}
                                color="bg-amber-400" // Orange
                                backgroundColor="bg-slate-500" // Gray
                            />
                            <span>{storageUsagePercent.toFixed(1)}%</span>
                        </div>
                    );
                })}
            </div>
        </div>
    )
}