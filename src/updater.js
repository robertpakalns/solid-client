import { check } from "@tauri-apps/plugin-updater"
import { message } from "@tauri-apps/plugin-dialog"

let checked = false

export const checkForAppUpdates = async (onUserClick = false) => {
    if (checked) return
    checked = true

    try {
        const update = await check()
        if (update) {
            const confirm = await message(`Update ${update.version} available. Update now?`, {
                title: "Update Available",
                okLabel: "Yes",
                cancelLabel: "Later"
            })
            if (confirm) await update.downloadAndInstall()

        }
        else if (onUserClick) message("You are on the latest version.", { title: "No Update Available" })

    }
    catch (error) {
        console.error("Update check failed:", error)
    }
    finally {
        checked = false
    }
}
