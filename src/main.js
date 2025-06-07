import { setKeybinding } from "./keybinding.js"
import { handleDeeplinks } from "./deeplink.js"
import { checkForAppUpdates } from "./updater.js"

setKeybinding()
handleDeeplinks()
checkForAppUpdates()