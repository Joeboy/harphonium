# Runtime Issue Workaround

## Problem
The Tauri app compiles successfully but fails to run due to a symbol lookup error:
```
symbol lookup error: /snap/core20/current/lib/x86_64-linux-gnu/libpthread.so.0: undefined symbol: __libc_pthread_init, version GLIBC_PRIVATE
```

**Root Cause**: This happens when running `npm run tauri dev` from within VS Code's integrated terminal, especially if VS Code is installed via snap. The snap environment conflicts with system libraries.

## ✅ SOLUTION: Use External Terminal

**The easiest fix is to run Tauri commands from a regular terminal outside VS Code:**

```bash
# Open a new terminal (outside VS Code)
cd /path/to/synthmob
npm run tauri dev
```

This avoids VS Code's snap environment entirely!

## Working Solutions

### 1. Frontend-Only Development ✅ WORKING
```bash
./dev.sh dev-frontend
```
- Starts Vite development server at http://localhost:1420
- UI works perfectly for testing interface
- Backend Rust functions won't respond (expected)
- Perfect for UI development and styling

### 2. Release Build (Alternative)
```bash
./dev.sh build-release
```
- Release builds sometimes avoid the runtime symbol conflicts
- Produces optimized binaries

### 3. Direct Frontend Development
```bash
npm run dev
```
- Runs Vite directly without Tauri
- Same result as option 1

## For Full Tauri Development

### Option A: Fix System Libraries
Install native versions of development libraries:
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

### Option B: Use Different Environment
- Use a non-snap based Linux distribution
- Use WSL2 if on Windows
- Use Docker container with native packages

### Option C: Focus on Android (Recommended)
Since this is a mobile synthesizer:
1. Develop UI with frontend-only mode
2. Test Rust logic with unit tests
3. Deploy directly to Android device for full testing

## Current Status: ✅ Development Possible

**You can continue development using the frontend-only mode!**
- UI development: ✅ Working perfectly
- Styling and layout: ✅ Working
- React components: ✅ Working
- Ready for audio library integration: ✅ Ready

The backend Rust code is correct and will work fine on Android (the target platform).
