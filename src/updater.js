import { invoke } from "@tauri-apps/api/core"
import { check } from "@tauri-apps/plugin-updater"
import { message } from "@tauri-apps/plugin-dialog"

export const checkForAppUpdates = async (onUserClick = false) => {
    const update = await check()
    if (update) {
        const confirm = await message(`Update ${update.version} available. Update now?`, { title: "Update Available", okLabel: "Yes", cancelLabel: "Later" })
        if (confirm) {
            await update.download()
            await invoke("restart")
        }
    } else if (onUserClick) {
        message("You are on the latest version.", { title: "No Update Available" })
    }
}