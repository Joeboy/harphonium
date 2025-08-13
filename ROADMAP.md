# SynthMob Roadmap

This document outlines the development progress and future plans for the SynthMob cross-platform synthesizer app.

## Phase 1: Basic Tauri App ✅ COMPLETED
- [x] Set up Tauri + React + Vite project structure
- [x] Configure for Android targeting (Tauri v2)
- [x] Create basic UI with frequency control
- [x] Set up Rust backend with basic command handlers
- [x] Create development scripts and documentation

## Phase 2: Cross-Platform Audio Architecture ✅ COMPLETED
- [x] **Multi-platform audio support**: Desktop (cpal) + Android (oboe)
- [x] **Audio module structure**: Cross-platform coordinator with platform-specific implementations
- [x] **Desktop audio engine**: Real-time audio synthesis using cpal
- [x] **Android audio engine**: Low-latency audio using oboe library
- [x] **C++ runtime integration**: Proper linking for Android native libraries
- [x] **Command interface**: `play_note` and `stop_note` Tauri commands
- [x] **Touch-friendly UI**: Mouse and touch event handling for mobile

## Phase 3: Android Deployment ✅ COMPLETED
- [x] **Android build system**: Proper NDK configuration and C++ linking
- [x] **APK generation**: Successful debug and release builds
- [x] **Emulator testing**: App runs on Android x86_64 emulator
- [x] **Native library loading**: Oboe integration working without crashes
- [x] **UI/Backend communication**: React frontend successfully calling Rust audio functions
- [x] **Development workflow**: Android dev mode and debug building

## Current Architecture (Implemented)

### Audio System Structure
```
src-tauri/src/audio/
├── mod.rs        # Cross-platform coordinator and state management
├── desktop.rs    # Desktop implementation using cpal
└── android.rs    # Android implementation using oboe
```

### Key Features Working
- **Cross-platform compilation**: Conditional compilation for desktop vs Android
- **Unified API**: Common interface (`play_frequency`, `stop_audio`) for both platforms
- **Touch-friendly UI**: React component with mouse and touch event handling
- **State management**: Atomic state tracking for frequency and playback status
- **Native integration**: Successful C++ runtime linking for Android

## Phase 4: Enhanced Synthesis Features (Next Priority)

### 4.1 Audio Synthesis Enhancement
- [ ] **Waveform types**: Add square, triangle, sawtooth waves beyond current sine wave
- [ ] **ADSR envelope**: Attack, Decay, Sustain, Release for note dynamics  
- [ ] **Volume control**: User-adjustable amplitude levels
- [ ] **Filter implementation**: Low-pass, high-pass, band-pass filters
- [ ] **Effects chain**: Basic reverb, delay, chorus effects

### 4.2 Polyphony Support
- [ ] **Multi-note playback**: Support simultaneous notes instead of single-note
- [ ] **Voice allocation**: Manage multiple concurrent audio streams
- [ ] **Note priority**: Handle voice stealing when max polyphony reached

## Phase 5: Enhanced User Interface

### 5.1 Virtual Piano Keyboard  
- [ ] **Piano layout**: Replace simple buttons with piano keyboard interface
- [ ] **Multi-touch support**: Allow multiple simultaneous key presses
- [ ] **Visual feedback**: Key press animations and active note indicators
- [ ] **Octave selection**: Controls to change keyboard range

### 5.2 Advanced Controls
- [ ] **Parameter sliders**: Real-time control for envelope, filter, effects
- [ ] **Preset system**: Save/load synthesizer configurations
- [ ] **Touch gestures**: Pitch bend, modulation wheel via touch/swipe

## Phase 6: Audio Quality & Performance

### 6.1 Audio Engine Optimization
- [ ] **Buffer tuning**: Optimize audio buffer sizes for latency vs stability
- [ ] **CPU profiling**: Monitor and optimize audio processing performance
- [ ] **Memory management**: Reduce allocations in real-time audio thread

### 6.2 Mobile Platform Features
- [ ] **Audio focus handling**: Proper Android audio session management
- [ ] **Background audio**: Continue playing when app backgrounded
- [ ] **Hardware integration**: Volume buttons, audio jack detection
- [ ] **Latency measurement**: Real-time audio latency monitoring and adjustment

## Current Implementation Status

### ✅ Working Components
- **Cross-platform builds**: Desktop (Linux/Windows/macOS) and Android (x86_64/aarch64)
- **React UI**: Touch-friendly frequency input and play/stop controls
- **Audio backends**: cpal (desktop) and oboe (Android) successfully integrated
- **Tauri commands**: `play_note` and `stop_note` working across platforms
- **Android deployment**: APK builds and installs successfully on emulator
- **C++ runtime**: Proper linking resolved Android crashes

### 🔧 Known Issues & Limitations
- **Audio output verification**: While audio functions execute successfully, actual sound output needs physical device testing
- **Single frequency mode**: Only one frequency can play at a time (no polyphony)
- **Basic waveform**: Currently limited to sine wave generation
- **No envelope shaping**: Abrupt note on/off without ADSR

### 📁 Current File Structure
```
synthmob/
├── src/                           # React frontend
│   ├── App.tsx                   # Main UI with frequency controls
│   └── main.tsx                  # React app entry point
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs              # Desktop binary entry point
│   │   ├── lib.rs               # Mobile library entry point  
│   │   └── audio/               # Cross-platform audio module
│   │       ├── mod.rs           # Audio coordination & state
│   │       ├── desktop.rs       # cpal implementation
│   │       └── android.rs       # oboe implementation
│   ├── build.rs                 # C++ linking configuration
│   ├── Cargo.toml              # Dependencies & targets
│   └── gen/android/            # Generated Android project files
└── README.md                    # Comprehensive setup documentation
```

## Next Immediate Steps (Priority Order)

1. **Audio output verification**: Test on physical Android device to confirm sound generation
2. **Waveform expansion**: Add square, triangle, sawtooth wave types beyond sine
3. **Volume control**: Add amplitude adjustment to UI and audio engine
4. **Piano keyboard UI**: Replace simple button with proper keyboard layout
5. **ADSR envelope**: Add attack/decay/sustain/release note dynamics

## Development Workflow

### Current Commands
- `npm run tauri dev` - Desktop development with hot reload
- `npm run tauri android dev` - Android development mode  
- `npm run tauri android build --target aarch64 --debug` - Android APK build
- `adb install -r path/to/app.apk` - Manual APK installation
- `adb logcat | grep synthmob` - Android app debugging

### Testing Procedures  
- **Desktop**: Direct audio output testing with system speakers/headphones
- **Android Emulator**: UI functionality testing (limited audio capabilities)
- **Physical Device**: Full audio output verification and performance testing

## Key Technical Achievements

### Solved Challenges ✅
1. **Cross-platform audio**: Successfully implemented cpal (desktop) + oboe (Android) architecture
2. **C++ runtime integration**: Resolved Android `__cxa_pure_virtual` crashes via proper linking in `build.rs`
3. **Tauri v2 mobile**: Successfully configured mobile targets and conditional compilation
4. **Android APK deployment**: Complete build pipeline from Rust/React to installable Android app
5. **Touch interface**: React UI works seamlessly on both desktop mouse and mobile touch

### Remaining Challenges 
1. **Audio latency optimization**: Fine-tune buffer sizes for minimal latency on mobile
2. **Physical device testing**: Verify actual audio output beyond emulator testing  
3. **Performance scaling**: Optimize for resource-constrained mobile hardware
4. **Cross-platform UI consistency**: Ensure UI works well across different screen sizes

## Contributing & Development

This project demonstrates a complete cross-platform audio application using modern Rust and web technologies. Key areas for contribution:

- **Audio synthesis**: Expand waveform types, filters, effects
- **UI/UX**: Design better mobile-first interface components  
- **Performance**: Audio engine optimization and profiling
- **Platform features**: Android-specific audio focus, background audio, hardware integration
- **Testing**: Comprehensive audio functionality testing across devices

## License

This project is for educational and demonstration purposes, showcasing cross-platform mobile development with Tauri v2, Rust, and React.
