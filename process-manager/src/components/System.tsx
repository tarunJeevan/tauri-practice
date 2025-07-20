import {SystemInfo} from "../types/system.ts";

export default function System({sysInfo}: { sysInfo: SystemInfo }) {
    return (
        <div>
            {/* TODO: Display system information */}
            {sysInfo.name}
        </div>
    )
}