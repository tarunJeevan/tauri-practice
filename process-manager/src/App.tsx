import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";

import {ProcessInfo} from "./types/process.ts";
// import {SystemInfo} from "./types/system.ts";
import "./App.css";

export default function App() {
    const [osName, setOSName] = useState<string>("");
    const [processes, setProcesses] = useState<ProcessInfo[]>([]);

    useEffect(() => {
        async function fetchData() {
            const os = await invoke<string>("get_os_name");
            const procs = await invoke<ProcessInfo[]>("list_processes");

            setOSName(os);
            setProcesses(procs);
        }

        fetchData();
    }, []);

    async function deleteProcess(id: string) {
        try {
            const success = await invoke<boolean>("kill_process_by_id", {id});
            if (success) {
                setProcesses(prevProcesses =>
                    prevProcesses.filter(p =>
                        p.id !== id
                    )
                );
            } else {
                // TODO: Do something else
            }
        } catch (error) {
            // TODO: Do anything else
            console.error(error);
        }
    }

    return (
        <main className="container">
            <h2>Operating System: {osName}</h2>

            <div className="process-list">
                {processes.length > 0 && processes.map(proc => (
                    <div key={proc.id} className="process">
                        <span>{proc.name} (ID: {proc.id})</span>
                        <button onClick={() => deleteProcess(proc.id)}>KILL</button>
                    </div>
                ))}
            </div>
        </main>
    );
}

