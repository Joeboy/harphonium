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
        
        # Check if Android device/emulator is connected
        DEVICE_COUNT=$($ANDROID_HOME/platform-tools/adb devices | grep -v "List of devices" | grep -v "^$" | wc -l)
        if [ "$DEVICE_COUNT" -eq 0 ]; then
            echo "No Android device detected. Starting emulator..."
            echo "Available AVDs:"
            $ANDROID_HOME/cmdline-tools/latest/bin/avdmanager list avd -c
            echo ""
            echo "Starting Medium_Phone_API_36 emulator..."
            $ANDROID_HOME/emulator/emulator -avd Medium_Phone_API_36 &
            EMULATOR_PID=$!
            echo "Emulator starting with PID: $EMULATOR_PID"
            echo "Waiting for device to be ready..."
            
            # Wait for device to appear (timeout after 60 seconds)
            TIMEOUT=60
            ELAPSED=0
            while [ "$ELAPSED" -lt "$TIMEOUT" ]; do
                DEVICE_COUNT=$($ANDROID_HOME/platform-tools/adb devices | grep -v "List of devices" | grep -v "^$" | wc -l)
                if [ "$DEVICE_COUNT" -gt 0 ]; then
                    echo "Device detected! Waiting for boot completion..."
                    $ANDROID_HOME/platform-tools/adb wait-for-device shell 'while [[ -z $(getprop sys.boot_completed) ]]; do sleep 1; done'
                    break
                fi
                sleep 2
                ELAPSED=$((ELAPSED + 2))
                echo "Waiting... ($ELAPSED/$TIMEOUT seconds)"
            done
            
            if [ "$ELAPSED" -ge "$TIMEOUT" ]; then
                echo "Error: Emulator failed to start within $TIMEOUT seconds"
                exit 1
            fi
        else
            echo "Android device detected:"
            $ANDROID_HOME/platform-tools/adb devices
        fi
        
        echo "Running Android development build..."
        npm run tauri android dev
        ;;
    "android-emulator")
        if [ -z "$ANDROID_HOME" ]; then
            echo "Error: ANDROID_HOME is not set. Please run './dev.sh android-setup' first"
            exit 1
        fi
        echo "Starting Android emulator..."
        echo "Available AVDs:"
        $ANDROID_HOME/cmdline-tools/latest/bin/avdmanager list avd -c
        echo ""
        echo "Starting Medium_Phone_API_36..."
        $ANDROID_HOME/emulator/emulator -avd Medium_Phone_API_36 &
        echo "Emulator started in background. Use 'adb devices' to check status."
        ;;
    "android-build")
        if [ -z "$ANDROID_HOME" ]; then
            echo "Error: ANDROID_HOME is not set. Please run './dev.sh android-setup' first"
            exit 1
        fi
        echo "Building Android APK (debug)..."
        npm run tauri android build --target aarch64 --debug
        ;;
    "android-install")
        if [ -z "$ANDROID_HOME" ]; then
            echo "Error: ANDROID_HOME is not set. Please run './dev.sh android-setup' first"
            exit 1
        fi
        APK_PATH="/home/joe/building/synthmob/src-tauri/gen/android/app/build/outputs/apk/aarch64/debug/app-aarch64-debug.apk"
        if [ -f "$APK_PATH" ]; then
            echo "Installing APK to connected device..."
            $ANDROID_HOME/platform-tools/adb install -r "$APK_PATH"
        else
            echo "APK not found at $APK_PATH"
            echo "Run './dev.sh android-build' first"
            exit 1
        fi
        ;;
    "android-logs")
        if [ -z "$ANDROID_HOME" ]; then
            echo "Error: ANDROID_HOME is not set. Please run './dev.sh android-setup' first"
            exit 1
        fi
        echo "Showing Android logs for SynthMob (press Ctrl+C to stop)..."
        $ANDROID_HOME/platform-tools/adb logcat | grep -E "(synthmob|Playing|Stopping|RustStdoutStderr)"
        ;;
    "android-status")
        if [ -z "$ANDROID_HOME" ]; then
            echo "Error: ANDROID_HOME is not set. Please run './dev.sh android-setup' first"
            exit 1
        fi
        echo "Android Development Status:"
        echo "=========================="
        echo "Connected devices:"
        $ANDROID_HOME/platform-tools/adb devices
        echo ""
        echo "SynthMob app processes:"
        $ANDROID_HOME/platform-tools/adb shell "ps | grep synthmob" || echo "No SynthMob processes running"
        echo ""
        echo "Available AVDs:"
        $ANDROID_HOME/cmdline-tools/latest/bin/avdmanager list avd -c
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
        echo "Usage: $0 {dev|dev-external|dev-frontend|build|build-release|android-setup|android-dev|android-emulator|android-build|android-install|android-logs|android-status|clean}"
        echo ""
        echo "Desktop Commands:"
        echo "  dev           - Start development server (desktop)"
        echo "  dev-external  - Show command for external terminal"
        echo "  dev-frontend  - Start frontend only (if desktop fails)"
        echo "  build         - Build production application (desktop)"
        echo "  build-release - Build release version (may avoid runtime issues)"
        echo ""
        echo "Android Commands:"
        echo "  android-setup    - Show Android development setup instructions"
        echo "  android-emulator - Start Android emulator"
        echo "  android-dev      - Start Android development (auto-starts emulator)"
        echo "  android-build    - Build Android APK (debug)"
        echo "  android-install  - Install built APK to device"
        echo "  android-logs     - Show real-time Android app logs"
        echo "  android-status   - Show Android development status"
        echo ""
        echo "Utility Commands:"
        echo "  clean         - Clean build artifacts"
        echo ""
        exit 1
        ;;
esac
