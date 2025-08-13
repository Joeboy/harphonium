# SynthMob Roadmap - Oboe Integration

This document outlines the steps to integrate Oboe for real-time audio synthesis in the SynthMob app.

## Phase 1: Basic Tauri App ✓ COMPLETED
- [x] Set up Tauri + React + Vite project structure
- [x] Configure for Android targeting (Tauri v2)
- [x] Create basic UI with frequency control
- [x] Set up Rust backend with basic command handlers
- [x] Create development scripts and documentation

## Phase 2: Audio Foundation (Next Steps)

### 2.1 Rust Audio Dependencies
Add audio processing crates to `src-tauri/Cargo.toml`:
```toml
[dependencies]
# ... existing dependencies
dasp = "0.11"           # Digital audio signal processing
cpal = "0.15"           # Cross-platform audio library
ringbuf = "0.3"         # Lock-free ring buffer for audio
```

### 2.2 Basic Audio Engine
Create `src-tauri/src/audio/` module:
- `mod.rs` - Public audio interface
- `synth.rs` - Oscillators and synthesis
- `engine.rs` - Audio callback management

### 2.3 Desktop Audio First
Implement audio synthesis using cpal for desktop development:
- Real-time audio output
- Basic sine wave oscillator
- Frequency control from frontend
- Volume control

## Phase 3: Android Native Audio (Oboe Integration)

### 3.1 Oboe Setup
- Add Oboe as a git submodule or use precompiled binaries
- Create C++ wrapper for Oboe functionality
- Set up CMake build for native components

### 3.2 FFI Bridge
Create Rust FFI bindings to call Oboe functions:
```rust
// src-tauri/src/oboe_bindings.rs
extern "C" {
    fn oboe_start_stream(sample_rate: i32, buffer_size: i32) -> i32;
    fn oboe_stop_stream() -> i32;
    fn oboe_write_audio(buffer: *const f32, frames: i32) -> i32;
}
```

### 3.3 Android Audio Engine
- Implement Android-specific audio backend using Oboe
- Low-latency audio processing
- Handle Android audio focus and lifecycle

## Phase 4: Advanced Synthesis Features

### 4.1 Oscillator Types
- Sine wave ✓ (basic)
- Square wave
- Triangle wave
- Sawtooth wave
- Noise generator

### 4.2 Synthesis Parameters
- ADSR envelope
- Filter (low-pass, high-pass, band-pass)
- LFO (Low Frequency Oscillator)
- Vibrato and tremolo

### 4.3 Polyphony
- Multiple simultaneous notes
- Voice allocation and management
- Note on/off handling

## Phase 5: User Interface Enhancements

### 5.1 Virtual Keyboard
- Piano keyboard layout
- Touch-friendly keys
- Visual feedback for pressed keys
- Multi-touch support

### 5.2 Parameter Controls
- Sliders and knobs for synthesis parameters
- Preset management
- Real-time parameter updates

### 5.3 Mobile-Optimized UI
- Responsive design for different screen sizes
- Touch gestures for parameter control
- Hardware back button handling

## Phase 6: Performance & Polish

### 6.1 Audio Optimization
- Buffer size optimization
- CPU usage monitoring
- Memory allocation optimization

### 6.2 Android Specific
- Audio latency measurement
- Background audio handling
- Audio focus management
- Integration with Android audio ecosystem

## Implementation Notes

### File Structure (Planned)
```
src-tauri/
├── src/
│   ├── main.rs
│   ├── audio/
│   │   ├── mod.rs
│   │   ├── synth.rs
│   │   ├── engine.rs
│   │   └── oboe_bridge.rs
│   ├── commands.rs
│   └── oboe_bindings.rs
├── oboe/              # Oboe C++ source (submodule)
├── native/
│   ├── oboe_wrapper.cpp
│   ├── oboe_wrapper.h
│   └── CMakeLists.txt
└── build.rs           # Build script for native components
```

### Key Challenges
1. **FFI Complexity**: Rust ↔ C++ ↔ Java/Kotlin integration
2. **Real-time Audio**: Meeting low-latency requirements
3. **Android Lifecycle**: Handling app state changes
4. **Cross-platform**: Desktop vs Android audio backends
5. **Performance**: Efficient audio processing on mobile hardware

## Getting Started with Phase 2

To continue development, the next immediate steps are:

1. Add audio dependencies to Cargo.toml
2. Implement basic audio engine with cpal for desktop
3. Test real-time synthesis on desktop
4. Then proceed to Android/Oboe integration

Each phase builds upon the previous one, ensuring a solid foundation before adding complexity.
