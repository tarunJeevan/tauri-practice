import {DiskInfo, SystemInfo} from "../types/system.ts";

export default function System({sysInfo, disks}: { sysInfo: SystemInfo, disks: DiskInfo[] }) {
    return (
        <div className="flex flex-row p-6">
            {/* Display system information */}
            <div>
                {sysInfo.name}
            </div>
            {/* Display all disk information */}
            <div className="flex flex-col p-2">
                {disks.map(disk => (
                    <div>
                        {disk.name}
                    </div>
                ))}
            </div>
        </div>
    )
}