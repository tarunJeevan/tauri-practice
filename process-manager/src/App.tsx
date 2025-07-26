import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";

import {ProcessInfo} from "./types/process.ts";
import {DiskInfo, SystemInfo} from "./types/system.ts";
import "./App.css";
import System from "./components/System.tsx";
import Processes from "./components/Processes.tsx";
import {listen, UnlistenFn} from "@tauri-apps/api/event";

export default function App() {
    const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
    const [disksInfo, setDisksInfo] = useState<DiskInfo[]>([]);
    const [processes, setProcesses] = useState<ProcessInfo[]>([]);

    // TODO: Add event to keep disks info updated
    useEffect(() => {
        let systemUnlisten: UnlistenFn;
        let processUnlisten: UnlistenFn;

        async function fetchData() {
            const disks = await invoke<DiskInfo[]>("get_all_disks");

            systemUnlisten = await listen<SystemInfo>('system_update', event => {
                setSystemInfo(event.payload);
            });
            processUnlisten = await listen<ProcessInfo[]>('process_list_update', event => {
                setProcesses(event.payload);
            })

            setDisksInfo(disks);
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

    return (
        <main className="container">
            <h2>Operating System: {systemInfo?.os}</h2>

            {/* TODO: Put System and Disk into one tab and Processes in another */}
            <div className="flex flex-col gap-6">
                {systemInfo !== null && disksInfo.length > 0 ? (
                    <System sysInfo={systemInfo} disks={disksInfo}/>
                ) : (
                    <div className="text-center text-gray-400 p-8">
                        Waiting for system info...
                    </div>
                )}

                {processes.length > 0 ? (
                    <Processes processes={processes}/>
                ) : (
                    <div className="text-center text-gray-400 p-8">
                        Waiting for process data...
                    </div>
                )}
            </div>
        </main>
    );
}

