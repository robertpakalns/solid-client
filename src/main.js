import { invoke } from "@tauri-apps/api/core"
import { setKeybinding } from "./keybinding.js"

setKeybinding()

document.addEventListener("DOMContentLoaded", () => {

    // External links
    document.body.addEventListener("click", async e => {
        const target = e.target.closest('a[target="_blank"]')
        if (!target) return
        e.preventDefault()
        await invoke("open_url", { url: target.href })
    })
})