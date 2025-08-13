#!/bin/bash

# SynthMob Development Scripts

case "$1" in
    "dev")
        echo "Starting Tauri development server..."
        echo "NOTE: If you get symbol lookup errors in VS Code terminal,"
        echo "      try running this command from a regular terminal outside VS Code"
        npm run tauri dev
        ;;
    "dev-external")
        echo "Use this command from an external terminal (not VS Code):"
        echo "  cd $(pwd)"
        echo "  npm run tauri dev"
        echo ""
        echo "This avoids VS Code snap environment conflicts."
        ;;
    "dev-frontend")
        echo "Starting frontend development server only..."
        echo "Frontend will be available at http://localhost:1420"
        echo "Note: Backend Rust functions won't work in this mode"
        npm run dev
        ;;
    "build-release")
        echo "Building release version (may avoid runtime issues)..."
        npm run tauri build
        ;;
    "build")
        echo "Building Tauri application..."
        npm run tauri build
        ;;
    "android-setup")
        echo "Setting up Android development..."
        echo "Please ensure ANDROID_HOME is set to your Android SDK path"
        echo "Example: export ANDROID_HOME=/home/\$USER/Android/Sdk"
        echo "Then run: npm run tauri android init"
        ;;
    "android-dev")
        if [ -z "$ANDROID_HOME" ]; then
            echo "Error: ANDROID_HOME is not set. Please run './dev.sh android-setup' first"
            exit 1
        fi
        echo "Running Android development build..."
        npm run tauri android dev
        ;;
    "clean")
        echo "Cleaning build artifacts..."
        cd src-tauri && cargo clean && cd ..
        rm -rf dist/
        rm -rf node_modules/.vite/
        echo "Clean complete"
        ;;
    *)
        echo "SynthMob Development Helper"
        echo ""
        echo "Usage: $0 {dev|dev-external|dev-frontend|build|build-release|android-setup|android-dev|clean}"
        echo ""
        echo "Commands:"
        echo "  dev           - Start development server (desktop)"
        echo "  dev-external  - Show command for external terminal"
        echo "  dev-frontend  - Start frontend only (if desktop fails)"
        echo "  build         - Build production application (desktop)"
        echo "  build-release - Build release version (may avoid runtime issues)"
        echo "  android-setup - Show Android development setup instructions"
        echo "  android-dev   - Start Android development (requires Android SDK)"
        echo "  clean         - Clean build artifacts"
        echo ""
        exit 1
        ;;
esac
