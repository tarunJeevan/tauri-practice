import {ProcessInfo} from "../types/process.ts";

export default function Processes({processes}: { processes: ProcessInfo[] }) {
    return (
        // TODO: Display all processes
        <div>{processes.map(proc => (
            <div>
                {proc.name}
            </div>
        ))}</div>
    )
}