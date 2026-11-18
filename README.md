# File Integrity Manager

A local desktop integrity application using **Rust, Tauri 2, React, and TypeScript**. Select a directory, create a SHA-256 manifest, and compare subsequent scans to find added, changed, and deleted files.

## Requirements
- Rust stable, Node.js 20+
- Tauri v2 prerequisites for your platform: https://v2.tauri.app/start/prerequisites/

## Run the desktop app
```bash
npm install
npm run tauri dev
```

## Test the Rust integrity engine
```bash
cd src-tauri && cargo test
```

The interface uses a Tauri folder chooser and invokes local Rust commands to snapshot and verify content. Manifests are saved as JSON in application memory for the current session. Scans are explicit; no background surveillance or uploads occur.

**Security notes:** This verifies content integrity but not authenticity. An attacker who controls both files and their manifest could alter both. A signed/trusted manifest would be required for authenticity; symlinks are not followed.

MIT licensed.
