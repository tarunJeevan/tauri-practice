import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";

import {ProcessInfo} from "./types/process.ts";
import {DiskInfo, SystemInfo} from "./types/system.ts";
import "./App.css";
import System from "./components/System.tsx";
import Disk from "./components/Disk.tsx";
import Processes from "./components/Processes.tsx";
import {listen, UnlistenFn} from "@tauri-apps/api/event";

export default function App() {
    const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
    const [diskInfo, setDiskInfo] = useState<DiskInfo[]>([]);
    const [processes, setProcesses] = useState<ProcessInfo[]>([]);

    useEffect(() => {
        let systemUnlisten: UnlistenFn;
        let processUnlisten: UnlistenFn;

        async function fetchData() {
            // const system = await invoke<SystemInfo>("get_sys_info");
            const disks = await invoke<DiskInfo[]>("get_all_disks");
            // const procs = await invoke<ProcessInfo[]>("list_processes");

            systemUnlisten = await listen<SystemInfo>('system_update', event => {
                setSystemInfo(event.payload);
            });
            processUnlisten = await listen<ProcessInfo[]>('process_list_update', event => {
                setProcesses(event.payload);
            })

            // setSystemInfo(system);
            setDiskInfo(disks);
            // setProcesses(procs);
        }

        void fetchData();

        return () => {
            // Clean up system updates listener
            if (systemUnlisten)
                systemUnlisten();
            // Clean up process updates listener
            if (processUnlisten)
                processUnlisten();

            // Tell backend to stop monitoring processes
            invoke("stop_monitoring_processes")
                .then(() => console.log("Stopping process monitoring on unmount..."))
                .catch(err => console.error(err));

            // Tell backend to stop monitoring system resource usage
            invoke("stop_monitoring_system")
                .then(() => console.log("Stopping system monitoring on unmount..."))
                .catch(err => console.error(err));
        }
    }, []);

    async function deleteProcess(id: string) {
        try {
            const success = await invoke<boolean>("try_kill_process_by_id", {id});

            if (success) {
                setProcesses(prevProcesses =>
                    prevProcesses.filter(p =>
                        p.id !== id
                    )
                );
            } else {
                // TODO: Do something else
                try {
                    const result = await invoke<null>("force_kill_process_by_id", {id});

                    if (result === null) {
                        setProcesses(prevProcesses =>
                            prevProcesses.filter(p =>
                                p.id !== id
                            )
                        );
                    } else {
                        // TODO: Display warning message to user?
                    }
                } catch (error) {
                    // Error returned by force_kill_process_by_id()
                    console.error(error);
                }
            }
        } catch (error) {
            // Error returned by try_kill_process_by_id()
            console.error(error);
        }
    }

    return (
        <main className="container">
            <h2>Operating System: {systemInfo?.os}</h2>

            {/* TODO: Put System and Disk into one tab and Processes in another */}
            <System sysInfo={systemInfo!}/>
            <Disk disks={diskInfo}/>
            <Processes processes={processes}/>

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

