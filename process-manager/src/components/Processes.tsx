import {invoke} from "@tauri-apps/api/core";
import {ProcessInfo} from "../types/process.ts";

export default function Processes({processes}: { processes: ProcessInfo[] }) {
    return (
        // TODO: Display all processes
        <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-700">
                <thead className="bg-gray-800">
                <tr>
                    <th scope="col" className="pr-3 pl-4 py-3 text-left text-sm font-semibold text-gray-300">
                        Process
                    </th>
                    <th scope="col" className="p-3 text-left text-sm font-semibold text-gray-300">
                        Status
                    </th>
                    <th scope="col" className="p-3 text-left text-sm font-semibold text-gray-300">
                        CPU (%)
                    </th>
                    <th scope="col" className="p-3 text-left text-sm font-semibold text-gray-300">
                        Memory
                    </th>
                </tr>
                </thead>
                <tbody>
                {processes.map(proc => (
                    <tr key={proc.id}>
                        <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-white">
                            {proc.name} (ID: {proc.id})
                        </td>
                        <td className="whitespace-nowrap py-4 px-3 text-sm text-gray-300">
                            {proc.status}
                        </td>
                        <td className="whitespace-nowrap py-4 px-3 text-sm text-gray-300">
                            {proc.cpu_usage_percent}%
                        </td>
                        <td className="whitespace-nowrap py-4 px-3 text-sm text-gray-300">
                            {proc.memory_used}
                        </td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    )
}

// TODO: Add delete process as a context menu option
async function deleteProcess(id: string) {
    try {
        const success = await invoke<boolean>("try_kill_process_by_id", {id});

        if (!success) {
            // TODO: Prompt user to confirm action to force kill process via dialog box
            console.log("Process could not be terminated.");
            // try {
            //     const result = await invoke<null>("force_kill_process_by_id", {id});
            //
            //     if (result === null) {
            //         setProcesses(prevProcesses =>
            //             prevProcesses.filter(p =>
            //                 p.id !== id
            //             )
            //         );
            //     } else {
            //         // TODO: Display warning message to user?
            //     }
            // } catch (error) {
            //     // Error returned by force_kill_process_by_id()
            //     console.error(error);
            // }
        }
    } catch (error) {
        // Error returned by try_kill_process_by_id()
        console.error(error);
    }
}