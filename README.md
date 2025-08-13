# SynthMob - Mobile Synthesizer

A mobile synthesizer app built with Tauri, React, and Rust, designed to run on Android devices.

## Features

- Simple synthesizer with frequency control
- Touch-friendly interface for mobile devices
- Real-time audio synthesis (to be implemented with Oboe)
- Built with Rust for performance-critical audio processing

## Development Setup

### Prerequisites

1. **Node.js** (v16 or later)
2. **Rust** (latest stable)
3. **Android SDK** (for Android development)

### Getting Started

1. Install dependencies:
   ```bash
   npm install
   ```

2. Run in development mode (desktop):
   ```bash
   npm run tauri dev
   ```
   
   **Important**: If you get symbol lookup errors in VS Code's integrated terminal, run the above command from a regular terminal outside VS Code. This is due to VS Code snap environment conflicts.

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
