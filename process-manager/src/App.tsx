import {useEffect, useState} from "react";

import {invoke} from "@tauri-apps/api/core";
import {listen, UnlistenFn} from "@tauri-apps/api/event";

import System from "./components/System.tsx";
import Processes from "./components/Processes.tsx";
import {ProcessInfo} from "./types/process.ts";
import {DiskInfo, SystemInfo} from "./types/system.ts";
import "./App.css";

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

            // Set event listeners
            systemUnlisten = await listen<SystemInfo>('system_update', event => {
                console.log("System update event firing");
                setSystemInfo(event.payload);
            });
            processUnlisten = await listen<ProcessInfo[]>('process_list_update', event => {
                console.log("Process update event firing");
                setProcesses(event.payload);
            });

            // Start system and process monitoring
            await invoke("monitor_sys_info");
            await invoke("monitor_processes");

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

