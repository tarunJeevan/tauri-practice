import {DiskInfo} from "../types/system.ts";

export default function Disk({disks}: { disks: DiskInfo[] }) {
    return (
        <div>{disks.map(disk => (
            <div>{disk.name}</div>
        ))}</div>
    )
}
