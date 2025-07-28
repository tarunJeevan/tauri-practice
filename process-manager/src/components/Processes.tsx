import {ProcessInfo} from "../types/process.ts";
// import React from "react";

// import {invoke} from "@tauri-apps/api/core";
// import {ask, message} from "@tauri-apps/plugin-dialog";
// import {Menu} from "@tauri-apps/api/menu/menu";
// import {listen, UnlistenFn} from "@tauri-apps/api/event";

// async function handleTerminate(id: string) {
//     alert(id);
//     // try {
//     //     const success = await invoke<boolean>("try_kill_process_by_id", {id});
//     //
//     //     if (!success) {
//     //         await message("Unable to terminate process. Try force-kill if absolutely necessary.",
//     //             {
//     //                 title: `Terminating Process ${id}`,
//     //                 kind: "info",
//     //             });
//     //     }
//     // } catch (error: any) {
//     //     // Error returned by try_kill_process_by_id()
//     //     console.error(error);
//     //     await message(error, {title: `Error Terminating Process ${id}`, kind: "error"});
//     // }
// }

// async function handleForceKill(id: string) {
//     alert(id);
//     // const confirm = await ask("Force-killing a process can be dangerous. Do you still want to proceed?",
//     //     {
//     //         title: `Force-Kill Process ${id}`,
//     //         kind: "warning"
//     //     });
//     //
//     // if (confirm) {
//     //     try {
//     //         await invoke<null>("force_kill_process_by_id", {id});
//     //     } catch (error: any) {
//     //         // Error returned by force_kill_process_by_id()
//     //         console.error(error);
//     //         await message(error, {title: `Error Force-Killing Process ${id}`, kind: "error"});
//     //     }
//     // }
// }

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
                {processes.map(proc => {
                    // const menuPromise = Menu.new({
                    //     items: [
                    //         {
                    //             id: `terminate-${proc.id}`,
                    //             text: 'Terminate',
                    //             action: (id) => handleTerminate(id)
                    //         },
                    //         {
                    //             id: `forcekill-${proc.id}`,
                    //             text: 'Force-Kill',
                    //             action: (id) => handleForceKill(id)
                    //         },
                    //     ],
                    // });
                    //
                    // React.useEffect(() => {
                    //     let unlisten: UnlistenFn;
                    //
                    //     async function handleMenuEvent() {
                    //         unlisten = await listen<string>('menu-event', event => {
                    //             const id = event.payload;
                    //
                    //             if (id == `terminate-${proc.id}`)
                    //                 invoke("try_kill_process");
                    //         });
                    //     }
                    //
                    //     void handleMenuEvent();
                    //
                    //     return () => {
                    //         if (unlisten)
                    //             unlisten();
                    //     }
                    // }, [proc.id]);
                    //
                    // const handleContextMenu = async (e: React.MouseEvent) => {
                    //     e.preventDefault();
                    //     const menu = await menuPromise;
                    //     await menu.popup();
                    // };

                    return (
                        // FIXME: Table columns shouldn't shift
                        <tr
                            key={proc.id}
                            // onContextMenu={handleContextMenu}
                        >
                            <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-white">
                                {proc.name} (ID: {proc.id})
                            </td>
                            <td className="whitespace-nowrap py-4 px-3 text-sm text-gray-300">
                                {proc.status}
                            </td>
                            <td className="whitespace-nowrap py-4 px-3 text-sm text-gray-300">
                                {proc.cpu_usage_percent.toFixed(1)}%
                            </td>
                            <td className="whitespace-nowrap py-4 px-3 text-sm text-gray-300">
                                {proc.memory_used}
                            </td>
                        </tr>
                    );
                })}
                </tbody>
            </table>
        </div>
    );
}