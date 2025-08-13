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
        
        TARGET=${2:-"aarch64"}
        echo "Building Android APK for target: $TARGET (debug)..."
        echo "Available targets: aarch64 (ARM64), armv7 (ARM32), i686 (x86), x86_64"
        
        # Clean old builds to ensure fresh APK
        echo "🧹 Cleaning previous builds..."
        rm -rf src-tauri/gen/android/app/build/outputs/apk/ 2>/dev/null || true
        
        # Record build start time
        BUILD_START=$(date +%s)
        
        npm run tauri android build -- --target "$TARGET" --debug
        
        if [ $? -eq 0 ]; then
            # Check multiple possible APK locations
            APK_CANDIDATES=(
                "src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk"
                "src-tauri/gen/android/app/build/outputs/apk/$TARGET/debug/app-$TARGET-debug.apk"
                "src-tauri/gen/android/app/build/outputs/apk/debug/app-debug.apk"
            )
            
            APK_PATH=""
            for candidate in "${APK_CANDIDATES[@]}"; do
                if [ -f "$candidate" ]; then
                    # Check if APK is newer than build start time
                    APK_TIME=$(stat -c %Y "$candidate" 2>/dev/null || echo "0")
                    if [ "$APK_TIME" -ge "$BUILD_START" ]; then
                        APK_PATH="$candidate"
                        echo "📱 Found fresh APK: $candidate"
                        break
                    else
                        echo "⚠️  Skipping old APK: $candidate (built before this run)"
                    fi
                fi
            done
            
            if [ -n "$APK_PATH" ]; then
                echo "✅ Fresh build successful! APK created at: $APK_PATH"
                echo "Size: $(du -h "$APK_PATH" | cut -f1)"
                echo "Built: $(date -d @$(stat -c %Y "$APK_PATH" 2>/dev/null || echo $(date +%s)))"
                echo ""
                echo "Next steps:"
                echo "  ./dev.sh android-install    # Install to connected device"
                echo "  ./dev.sh android-logs       # Monitor app logs"
                echo "  ./dev.sh android-logs-clean # Clear logs buffer" 
                echo "  ./dev.sh android-logs-native # Monitor native touch logs"
            else
                echo "❌ Build completed but no fresh APK found at expected locations"
                echo "Available APK files:"
                find src-tauri/gen/android -name "*.apk" -type f -exec ls -la {} \; 2>/dev/null || echo "None found"
                echo ""
                echo "💡 Note: APKs older than this build are ignored to prevent using stale builds"
            fi
        else
            echo "❌ Build failed. Check error messages above."
        fi
        ;;
    "android-install")
        if [ -z "$ANDROID_HOME" ]; then
            echo "Error: ANDROID_HOME is not set. Please run './dev.sh android-setup' first"
            exit 1
        fi
        
        # Check if device is connected
        DEVICE_COUNT=$($ANDROID_HOME/platform-tools/adb devices | grep -v "List of devices" | grep -v "^$" | wc -l)
        if [ "$DEVICE_COUNT" -eq 0 ]; then
            echo "❌ No Android device detected. Please connect your device and enable USB debugging."
            echo "Run './dev.sh android-status' to check device connection."
            exit 1
        fi
        
        # Look for available APK files
        APK_PATTERN="src-tauri/gen/android/app/build/outputs/apk/*/debug/app-*-debug.apk"
        APK_FILES=($(ls $APK_PATTERN 2>/dev/null))
        
        if [ ${#APK_FILES[@]} -eq 0 ]; then
            echo "❌ No APK files found. Please build first with:"
            echo "  ./dev.sh android-build"
            exit 1
        elif [ ${#APK_FILES[@]} -eq 1 ]; then
            APK_PATH="${APK_FILES[0]}"
        else
            echo "Multiple APK files found. Select one:"
            for i in "${!APK_FILES[@]}"; do
                echo "  $((i+1))) $(basename "${APK_FILES[i]}")"
            done
            read -p "Enter choice (1-${#APK_FILES[@]}): " choice
            if [[ "$choice" =~ ^[0-9]+$ ]] && [ "$choice" -ge 1 ] && [ "$choice" -le "${#APK_FILES[@]}" ]; then
                APK_PATH="${APK_FILES[$((choice-1))]}"
            else
                echo "Invalid choice. Exiting."
                exit 1
            fi
        fi
        
        echo "Installing APK: $(basename "$APK_PATH")"
        echo "Size: $(du -h "$APK_PATH" | cut -f1)"
        echo "Installing to:"
        $ANDROID_HOME/platform-tools/adb devices
        
        if $ANDROID_HOME/platform-tools/adb install -r "$APK_PATH"; then
            echo "✅ Installation successful!"
            echo ""
            echo "Next steps:"
            echo "  ./dev.sh android-logs       # Monitor app logs"
            echo "  ./dev.sh android-logs-clean # Clear logs buffer" 
            echo "  ./dev.sh android-logs-native # Monitor native touch logs"
            echo "  # Launch app manually on device, or:"
            echo "  adb shell am start -n uk.co.joebutton.synthmob/.MainActivity"
        else
            echo "❌ Installation failed. Try:"
            echo "  adb uninstall uk.co.joebutton.synthmob"
            echo "  ./dev.sh android-install"
        fi
        ;;
    "android-logs")
        if [ -z "$ANDROID_HOME" ]; then
            echo "Error: ANDROID_HOME is not set. Please run './dev.sh android-setup' first"
            exit 1
        fi
        
        # Default timeout
        timeout_duration=10
        clean_first=false
        
        # Parse all arguments for flags
        shift # Remove the command name
        while [ $# -gt 0 ]; do
            case $1 in
                --clean|-c)
                    clean_first=true
                    ;;
                --timeout=*)
                    timeout_duration="${1#*=}"
                    ;;
                -t=*)
                    timeout_duration="${1#*=}"
                    ;;
                --timeout|-t)
                    shift
                    timeout_duration="$1"
                    ;;
            esac
            shift
        done
        
        if [ "$clean_first" = true ]; then
            echo "🧹 Clearing Android logs..."
            $ANDROID_HOME/platform-tools/adb logcat -c
            echo "✅ Android logs cleared"
        fi
        
        echo "📱 Showing Android logs for SynthMob (${timeout_duration}s timeout)..."
        timeout "${timeout_duration}" $ANDROID_HOME/platform-tools/adb logcat | grep -E "(synthmob|Playing|Stopping|RustStdoutStderr|Buffer|Performance|Oboe)" || echo "⏱️ Log monitoring timed out"
        ;;
    "android-logs-clean")
        if [ -z "$ANDROID_HOME" ]; then
            echo "Error: ANDROID_HOME is not set. Please run './dev.sh android-setup' first"
            exit 1
        fi
        echo "Clearing Android logs..."
        $ANDROID_HOME/platform-tools/adb logcat -c
        echo "✅ Android logs cleared"
        ;;
    "android-logs-native")
        if [ -z "$ANDROID_HOME" ]; then
            echo "Error: ANDROID_HOME is not set. Please run './dev.sh android-setup' first"
            exit 1
        fi
        
        # Default timeout
        timeout_duration=10
        clean_first=false
        
        # Parse all arguments for flags
        shift # Remove the command name
        while [ $# -gt 0 ]; do
            case $1 in
                --clean|-c)
                    clean_first=true
                    ;;
                --timeout=*)
                    timeout_duration="${1#*=}"
                    ;;
                -t=*)
                    timeout_duration="${1#*=}"
                    ;;
                --timeout|-t)
                    shift
                    timeout_duration="$1"
                    ;;
            esac
            shift
        done
        
        if [ "$clean_first" = true ]; then
            echo "🧹 Clearing Android logs..."
            $ANDROID_HOME/platform-tools/adb logcat -c
            echo "✅ Android logs cleared"
        fi
        
        echo "🎯 Showing native touch handler logs (${timeout_duration}s timeout)..."
        timeout "${timeout_duration}" $ANDROID_HOME/platform-tools/adb logcat | grep -E "(NativeTouch|MainActivity.*overlay|CALLING NATIVE|Native note trigger)" || echo "⏱️ Log monitoring timed out"
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
    "android-device-test")
        if [ -z "$ANDROID_HOME" ]; then
            echo "Error: ANDROID_HOME is not set. Please run './dev.sh android-setup' first"
            exit 1
        fi
        echo "🔧 Real Android Device Testing Workflow"
        echo "======================================="
        echo ""
        echo "Step 1: Check device connection..."
        DEVICE_COUNT=$($ANDROID_HOME/platform-tools/adb devices | grep -v "List of devices" | grep -v "^$" | wc -l)
        if [ "$DEVICE_COUNT" -eq 0 ]; then
            echo "❌ No device detected. Please:"
            echo "  1. Connect Android device via USB"
            echo "  2. Enable Developer Options (tap Build Number 7 times)"
            echo "  3. Enable USB Debugging in Developer Options"
            echo "  4. Allow debugging when prompted on device"
            exit 1
        else
            echo "✅ Device(s) detected:"
            $ANDROID_HOME/platform-tools/adb devices
        fi
        echo ""
        
        echo "Step 2: Building APK for real device..."
        npm run tauri android build -- --target aarch64 --debug
        
        if [ $? -eq 0 ]; then
            echo "✅ Build successful!"
        else
            echo "❌ Build failed. Please fix build errors and try again."
            exit 1
        fi
        echo ""
        
        echo "Step 3: Installing APK to device..."
        
        # Look for available APK files in order of preference
        APK_CANDIDATES=(
            "src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk"
            "src-tauri/gen/android/app/build/outputs/apk/aarch64/debug/app-aarch64-debug.apk"
            "src-tauri/gen/android/app/build/outputs/apk/armv7/debug/app-armv7-debug.apk"
            "src-tauri/gen/android/app/build/outputs/apk/x86_64/debug/app-x86_64-debug.apk"
        )
        
        APK_PATH=""
        for candidate in "${APK_CANDIDATES[@]}"; do
            if [ -f "$candidate" ]; then
                APK_PATH="$candidate"
                echo "Found APK: $(basename "$APK_PATH")"
                echo "Size: $(du -h "$APK_PATH" | cut -f1)"
                break
            fi
        done
        
        if [ -z "$APK_PATH" ]; then
            echo "❌ No APK found. Available APK files:"
            find src-tauri/gen/android -name "*.apk" -type f 2>/dev/null || echo "None found"
            exit 1
        fi
        
        if $ANDROID_HOME/platform-tools/adb install -r "$APK_PATH"; then
            echo "✅ Installation successful!"
        else
            echo "❌ Installation failed"
            exit 1
        fi
        echo ""
        
        echo "Step 4: Launching app..."
        $ANDROID_HOME/platform-tools/adb shell am start -n uk.co.joebutton.synthmob/.MainActivity
        echo "✅ App launched on device"
        echo ""
        
        echo "🎵 AUDIO TESTING CHECKLIST:"
        echo "=========================="
        echo "□ App appears on device screen"
        echo "□ Frequency input field works"
        echo "□ Play Note button is responsive"
        echo "□ **ACTUAL SOUND** comes from device speakers/headphones"
        echo "□ Different frequencies produce different pitches"
        echo "□ Stop functionality works"
        echo "□ No crashes or freezing"
        echo ""
        echo "📱 Test different frequencies:"
        echo "   220 Hz (low tone)"
        echo "   440 Hz (middle A)"
        echo "   880 Hz (high tone)"
        echo ""
        echo "📋 Monitoring logs in separate terminal:"
        echo "   ./dev.sh android-logs         # General app logs"
        echo "   ./dev.sh android-logs-native  # Native touch logs"
        echo "   ./dev.sh android-logs-clean   # Clear log buffer"
        echo ""
        echo "See TESTING_REAL_DEVICE.md for complete testing guide."
        ;;
    "clean")
        echo "Cleaning build artifacts..."
        cd src-tauri && cargo clean && cd ..
        rm -rf dist/
        rm -rf node_modules/.vite/
        rm -rf src-tauri/gen/
        echo "Clean complete"
        ;;
    *)
        echo "SynthMob Development Helper"
        echo ""
        echo "Usage: $0 {dev|dev-external|dev-frontend|build|build-release|android-setup|android-dev|android-emulator|android-build|android-install|android-device-test|android-logs|android-logs-clean|android-logs-native|android-status|clean}"
        echo ""
        echo "Desktop Commands:"
        echo "  dev           - Start development server (desktop)"
        echo "  dev-external  - Show command for external terminal"
        echo "  dev-frontend  - Start frontend only (if desktop fails)"
        echo "  build         - Build production application (desktop)"
        echo "  build-release - Build release version (may avoid runtime issues)"
        echo ""
        echo "Android Commands:"
        echo "  android-setup       - Show Android development setup instructions"
        echo "  android-emulator    - Start Android emulator"
        echo "  android-dev         - Start Android development (auto-starts emulator)"
        echo "  android-build       - Build Android APK (debug)"
        echo "  android-install     - Install built APK to device"
        echo "  android-device-test - Complete real device testing workflow"
        echo "  android-logs        - Show real-time Android app logs with timeout (use --clean to clear first, --timeout=30 to set duration)"
        echo "  android-logs-clean  - Clear Android logs buffer"
        echo "  android-logs-native - Show native touch handler logs with timeout (use --clean to clear first, --timeout=30 to set duration)"
        echo "  android-status      - Show Android development status"
        echo ""
        echo "Utility Commands:"
        echo "  clean         - Clean build artifacts"
        echo ""
        exit 1
        ;;
esac
