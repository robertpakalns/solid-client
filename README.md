# Solid Client
An unofficial desktop client for Vectaria.io game

### Features
* Deeplinks
* Developer Tools (`F12`)
* Discord RPC Support
* Fullscreen (`F11`)

### Protocol
Solid Client uses `solid://` protocol to launch the client and prevents multiple instances from running simultaneously.  
Syntax: `solid://?url=/path/to/page`, `solid://?url=path/to/page`, or just `solid://`.

### Dependencies
Solid Client is based on `Tauri` (`Rust`) framework and `Vite` (`Node.js`) frontend bundler. For more information, check `src-tauri/Cargo.toml` and `package.json`.

by robertpakalns | [Community Server](https://discord.gg/yPjrUrvSzv) | Powered by Tricko