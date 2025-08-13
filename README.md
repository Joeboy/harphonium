# SynthMob - Cross-Platform Synthesizer

A cross-platform synthesizer app built with Tauri, React, and Rust that runs on both desktop and Android with low-latency audio support.

## Features

- 🎵 **Cross-platform audio synthesis** - Desktop (cpal) and Android (oboe)
- 📱 **Touch-friendly React UI** - Responsive synthesizer interface
- ⚡ **Low-latency audio** - Optimized for real-time audio synthesis
- 🦀 **Rust backend** - High-performance audio processing
- 📦 **Single codebase** - Runs on desktop and mobile from same source

## Architecture

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust with Tauri framework
- **Audio**: 
  - Desktop: `cpal` library for cross-platform audio
  - Android: `oboe` library for low-latency audio via OpenSL ES
- **Build System**: Tauri v2 with mobile support

## Prerequisites

### For Desktop Development
- Node.js (18+)
- Rust (latest stable)
- System audio libraries (ALSA/PulseAudio on Linux)

### For Android Development
- All desktop prerequisites
- Android SDK (API level 24+)
- Android NDK (25.2.9519653 recommended)
- Java 17 JDK

## Environment Setup

### 1. Install Rust and Tauri CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Tauri CLI
cargo install tauri-cli --version "^2.0.0"
```

### 2. Android Development Setup

```bash
# Set environment variables (add to ~/.bashrc or ~/.zshrc)
export ANDROID_HOME=/home/$USER/Android/Sdk
export NDK_HOME=$ANDROID_HOME/ndk/25.2.9519653
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64

# Add to PATH
export PATH=$PATH:$ANDROID_HOME/platform-tools:$ANDROID_HOME/tools
```

### 3. Install Android Build Targets

```bash
# Add Android targets for Rust
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

### 4. Install Dependencies

```bash
cd synthmob
npm install
```

## Running the App

### Desktop Development

```bash
# Start development server with hot reload
npm run tauri dev

# Build desktop app
npm run tauri build
```

**Important**: If you get symbol lookup errors in VS Code's integrated terminal, run the above command from a regular terminal outside VS Code. This is due to VS Code snap environment conflicts.

### Android Development

#### Setting up Android Emulator

1. **Create an AVD** (Android Virtual Device):
   ```bash
   # List available system images
   $ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager --list | grep system-images
   
   # Create AVD with API 36
   $ANDROID_HOME/cmdline-tools/latest/bin/avdmanager create avd \
     -n Medium_Phone_API_36 \
     -k "system-images;android-36;google_apis;x86_64" \
     -d "pixel_7"
   ```

2. **Start the emulator**:
   ```bash
   # Start emulator in background
   $ANDROID_HOME/emulator/emulator -avd Medium_Phone_API_36 &
   
   # Verify device is connected
   adb devices
   ```

#### Building for Android

**Important**: Use debug builds for testing. Development mode doesn't work reliably on Android.

```bash
# Build and install debug APK
npm run tauri android build --target aarch64 --debug

# Or build without installing
npm run tauri android build --target aarch64 --debug --no-bundle
```

#### Installing on Emulator

```bash
# Manual installation (if auto-install fails)
adb install src-tauri/gen/android/app/build/outputs/apk/debug/app-debug.apk

# Check app installation
adb shell pm list packages | grep com.synthmob.app
```

## Testing the Audio

### Desktop
The app should produce audio immediately when clicking frequency buttons. If no audio:
- Check system volume
- Verify audio device permissions
- Try different audio backends (ALSA/PulseAudio on Linux)

### Android
Audio testing on Android emulator:
- **Note**: Android emulator may not produce audible sound
- Check logcat for audio initialization messages:
  ```bash
  adb logcat | grep -E "(synthmob|oboe|audio)"
  ```
- For actual audio testing, use a physical device

## Project Structure

```
synthmob/
├── src/                    # React frontend
│   ├── App.tsx            # Main UI component
│   └── main.tsx           # React entry point
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs         # Mobile library entry
│   │   ├── main.rs        # Desktop binary entry  
│   │   └── audio/         # Audio modules
│   │       ├── mod.rs     # Cross-platform coordinator
│   │       ├── desktop.rs # Desktop audio (cpal)
│   │       └── android.rs # Android audio (oboe)
│   ├── build.rs           # Build configuration
│   ├── Cargo.toml         # Rust dependencies
│   └── gen/android/       # Generated Android files
└── README.md              # This file
```

## Troubleshooting

### Common Issues

1. **Symbol lookup errors in VS Code terminal**
   - **Solution**: Run commands in external terminal, not VS Code integrated terminal

2. **C++ runtime errors on Android** (`__cxa_pure_virtual`)
   - **Solution**: Already fixed via `build.rs` C++ library linking

3. **Audio not working on desktop**
   - Check audio permissions and system audio settings
   - Try running from different terminal environment

4. **Android build fails**
   - Verify ANDROID_HOME and NDK_HOME are set correctly
   - Check Java version: `java -version` (should be 17)
   - Ensure Android targets are installed: `rustup target list --installed`

5. **Emulator connection issues**
   - Restart adb: `adb kill-server && adb start-server`
   - Check emulator is running: `adb devices`

6. **Development server won't start**
   - Clear node_modules: `rm -rf node_modules && npm install`
   - Clear Tauri cache: `cargo clean` in src-tauri directory

### Getting Logs

**Desktop logs**:
```bash
# Run with debug output
RUST_LOG=debug npm run tauri dev
```

**Android logs**:
```bash
# Monitor app logs
adb logcat | grep -E "(synthmob|Tauri)"

# Monitor audio-specific logs  
adb logcat | grep -E "(oboe|audio|AudioStream)"
```

## Development Workflow

1. **Desktop-first development**: Start with `npm run tauri dev` for rapid iteration
2. **Test Android builds periodically**: Use debug builds for faster compilation
3. **Use external terminal**: Avoid VS Code integrated terminal for Tauri commands
4. **Monitor logs**: Use `adb logcat` to debug Android-specific issues
5. **Cross-platform testing**: Test both desktop and Android to ensure compatibility

## Contributing

This project demonstrates cross-platform audio with Tauri v2. Key areas for enhancement:
- Additional waveforms (square, triangle, sawtooth)
- Audio effects (reverb, delay, filters)
- MIDI input support
- Recording capabilities
- More sophisticated synthesis algorithms

## License

This project is for educational and demonstration purposes.

### Android Development Setup

To build and run on Android, you need to set up the Android environment:

1. **Install Android Studio** and the Android SDK
2. **Set environment variables**:
   ```bash
   export ANDROID_HOME=/path/to/your/android/sdk
   export NDK_HOME=$ANDROID_HOME/ndk/[version]
   ```

3. **Add Android targets**:
   ```bash
   rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
   ```

4. **Initialize Android project** (after setting ANDROID_HOME):
   ```bash
   npm run tauri android init
   ```

5. **Run on Android**:
   ```bash
   npm run tauri android dev
   ```

## Project Structure

```
synthmob/
├── src/                 # React frontend
│   ├── App.tsx         # Main React component
│   ├── main.tsx        # React entry point
│   └── *.css          # Styles
├── src-tauri/          # Rust backend
│   ├── src/
│   │   └── main.rs     # Tauri app logic
│   ├── Cargo.toml      # Rust dependencies
│   └── tauri.conf.json # Tauri configuration
└── index.html          # HTML entry point
```

## Audio Synthesis (Roadmap)

The app is designed to integrate with Oboe (Android's high-performance audio library) for real-time audio synthesis:

1. **Oboe Integration**: Use FFI to call Oboe from Rust
2. **Synthesis Engine**: Implement basic waveform generators (sine, square, triangle, sawtooth)
3. **Touch Interface**: Add virtual keyboard and parameter controls
4. **Effects**: Add basic effects like reverb, delay, and filters

## Building for Production

### Desktop
```bash
npm run tauri build
```

### Android
```bash
npm run tauri android build
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
