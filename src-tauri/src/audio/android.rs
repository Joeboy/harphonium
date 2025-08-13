// Android audio implementation using oboe with FunDSP integration
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use fundsp::hacker::{sine_hz, An, Pipe, Constant, Sine, U1};

pub fn initialize_audio_engine(
    is_playing: Arc<AtomicBool>,
    frequency_bits: Arc<AtomicU32>,
) -> Result<(), Box<dyn std::error::Error>> {
    use oboe::{
        AudioOutputCallback, AudioOutputStreamSafe, AudioStream, AudioStreamBase,
        AudioStreamBuilder, AudioStreamSafe, DataCallbackResult, PerformanceMode, SharingMode,
    };

    println!("Initializing Android audio engine with Oboe - CALLBACK MODE");

    // Create callback handler with ultra-low latency processing using FunDSP
    struct AudioCallback {
        // FunDSP synthesizer state - using the actual return type of sine_hz
        synth: An<Pipe<Constant<U1>, Sine>>,
        current_frequency: f64,
        // Legacy state for smooth transition  
        phase: f32,
        envelope: f32,
        sample_rate: f32,
        fade_step: f32,
        is_playing: Arc<AtomicBool>,
        frequency_bits: Arc<AtomicU32>,
        // FunDSP integration flag
        use_fundsp: bool,
    }

    impl AudioOutputCallback for AudioCallback {
        type FrameType = (f32, oboe::Mono); // Correct frame type for mono

        fn on_audio_ready(
            &mut self,
            _stream: &mut dyn AudioOutputStreamSafe,
            frames: &mut [f32],
        ) -> DataCallbackResult {
            let playing = self.is_playing.load(Ordering::Relaxed);
            let raw_frequency = f32::from_bits(self.frequency_bits.load(Ordering::Relaxed));
            
            // CRITICAL: Validate frequency to prevent audio callback panics
            let frequency = if raw_frequency.is_finite() && raw_frequency > 0.0 && raw_frequency < 20000.0 {
                raw_frequency
            } else {
                0.0  // Safe fallback
            };

            if self.use_fundsp && playing && frequency > 0.0 {
                // FunDSP audio generation path
                // Update frequency if it changed
                let new_freq = frequency as f64;
                if (new_freq - self.current_frequency).abs() > 0.1 {
                    self.current_frequency = new_freq;
                    // Create a new sine oscillator with the updated frequency
                    self.synth = sine_hz(new_freq as f32);
                    self.synth.set_sample_rate(self.sample_rate as f64);
                }
                
                // Generate audio using FunDSP
                for sample in frames.iter_mut() {
                    // Process one sample through FunDSP
                    let output = if let Ok(result) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        self.synth.get_mono()
                    })) {
                        result
                    } else {
                        // If FunDSP panics, fall back to silent output
                        self.use_fundsp = false; // Disable FunDSP for this stream
                        0.0
                    };
                    
                    // Safety: ensure output is finite and in valid range
                    let output = output as f32;
                    *sample = if output.is_finite() && output.abs() <= 1.0 {
                        output * 0.5 // Apply volume scaling
                    } else {
                        0.0 // Safety fallback
                    };
                }
            } else {
                // Legacy audio generation path (fallback)
                if playing && frequency > 0.0 {
                    // Instant attack - fade in quickly
                    if self.envelope < 1.0 {
                        self.envelope = (self.envelope + self.fade_step * 4.0).min(1.0);
                        // 4x faster attack
                    }

                    // Generate sine wave directly in callback for mono
                    for sample in frames.iter_mut() {
                        let sine_val = (self.phase * 2.0 * std::f32::consts::PI).sin();
                        // Additional safety: ensure sine result is finite
                        *sample = if sine_val.is_finite() { 
                            sine_val * 0.5 * self.envelope 
                        } else { 
                            0.0 
                        };

                        self.phase += frequency / self.sample_rate;
                        // Ensure phase stays in valid range
                        if self.phase >= 1.0 {
                            self.phase -= 1.0;
                        } else if !self.phase.is_finite() {
                            self.phase = 0.0; // Reset if phase becomes invalid
                        }
                    }
                } else {
                    // Fade out
                    if self.envelope > 0.0 {
                        self.envelope = (self.envelope - self.fade_step).max(0.0);

                        for sample in frames.iter_mut() {
                            if self.envelope > 0.0 {
                                let sine_val = (self.phase * 2.0 * std::f32::consts::PI).sin();
                                *sample = if sine_val.is_finite() {
                                    sine_val * 0.5 * self.envelope
                                } else {
                                    0.0
                                };

                                self.phase += frequency / self.sample_rate;
                                if self.phase >= 1.0 {
                                    self.phase -= 1.0;
                                } else if !self.phase.is_finite() {
                                    self.phase = 0.0; // Reset if phase becomes invalid
                                }
                            } else {
                                *sample = 0.0;
                                self.phase = 0.0; // Reset phase when silent
                            }
                        }
                    } else {
                        // Silence
                        for sample in frames.iter_mut() {
                            *sample = 0.0;
                        }
                        self.phase = 0.0;
                    }
                }
            }

            DataCallbackResult::Continue
        }
    }

    // Try CALLBACK MODE with ultra-aggressive latency settings
    let buffer_sizes = [16, 24, 32, 48, 64]; // Even smaller buffers for callback mode
    let sample_rates = [48000, 44100]; // Prefer higher sample rates for callback mode
    let mut stream = None;
    let mut attempt_info = Vec::new();
    let mut best_latency = f32::MAX;
    let mut final_sample_rate = 48000.0;

    // Helper function to create FunDSP synthesizer
    let create_fundsp_synth = |sample_rate: f32| -> Result<An<Pipe<Constant<U1>, Sine>>, Box<dyn std::error::Error>> {
        // Start with a simple sine wave oscillator at a fixed frequency
        // We'll modulate the frequency in the callback
        let mut synth = sine_hz(440.0);
        synth.set_sample_rate(sample_rate as f64);
        Ok(synth)
    };

    // Strategy 1: CALLBACK + LowLatency + Exclusive - most aggressive
    for &sample_rate in &sample_rates {
        for &buffer_size in &buffer_sizes {
            // Try to create FunDSP synth first
            let use_fundsp = match create_fundsp_synth(sample_rate as f32) {
                Ok(synth) => {
                    let callback = AudioCallback {
                        synth,
                        current_frequency: 440.0,
                        phase: 0.0,
                        envelope: 0.0,
                        sample_rate: sample_rate as f32,
                        fade_step: 1.0 / (sample_rate as f32 * 0.0005), // 0.5ms fade (very fast)
                        is_playing: is_playing.clone(),
                        frequency_bits: frequency_bits.clone(),
                        use_fundsp: true,
                    };
                    Some(callback)
                }
                Err(e) => {
                    println!("⚠️ FunDSP init failed: {}, using fallback", e);
                    let callback = AudioCallback {
                        synth: sine_hz(440.0), // Dummy value
                        current_frequency: 440.0,
                        phase: 0.0,
                        envelope: 0.0,
                        sample_rate: sample_rate as f32,
                        fade_step: 1.0 / (sample_rate as f32 * 0.0005),
                        is_playing: is_playing.clone(),
                        frequency_bits: frequency_bits.clone(),
                        use_fundsp: false,
                    };
                    Some(callback)
                }
            };

            if let Some(callback) = use_fundsp {
                match AudioStreamBuilder::default()
                    .set_format::<f32>()
                    .set_channel_count::<oboe::Mono>()
                    .set_sample_rate(sample_rate)
                    .set_frames_per_callback(buffer_size)
                    .set_performance_mode(PerformanceMode::LowLatency)
                    .set_sharing_mode(SharingMode::Exclusive)
                    .set_callback(callback)
                    .open_stream()
                {
                    Ok(s) => {
                        let actual_frames = s.get_frames_per_callback();
                        let latency_ms = (actual_frames as f32 / sample_rate as f32) * 1000.0;
                        attempt_info.push(format!(
                            "🔥 CALLBACK+LowLatency+Exclusive {}Hz {}→{} frames ({:.2}ms)",
                            sample_rate, buffer_size, actual_frames, latency_ms
                        ));

                        if latency_ms < best_latency {
                            best_latency = latency_ms;
                            final_sample_rate = sample_rate as f32;
                            stream = Some(s);
                        }
                    }
                    Err(e) => {
                        attempt_info.push(format!(
                            "CALLBACK+LowLatency+Exclusive {}Hz {} failed: {}",
                            sample_rate, buffer_size, e
                        ));
                    }
                }
            }
        }
    }

    // Strategy 2: CALLBACK + PowerSaving + Exclusive
    if stream.is_none() {
        for &sample_rate in &sample_rates {
            for &buffer_size in &buffer_sizes {
                let use_fundsp = match create_fundsp_synth(sample_rate as f32) {
                    Ok(synth) => {
                        let callback = AudioCallback {
                            synth,
                            current_frequency: 440.0,
                            phase: 0.0,
                            envelope: 0.0,
                            sample_rate: sample_rate as f32,
                            fade_step: 1.0 / (sample_rate as f32 * 0.0005),
                            is_playing: is_playing.clone(),
                            frequency_bits: frequency_bits.clone(),
                            use_fundsp: true,
                        };
                        Some(callback)
                    }
                    Err(_) => {
                        let callback = AudioCallback {
                            synth: sine_hz(440.0), // Dummy value
                            current_frequency: 440.0,
                            phase: 0.0,
                            envelope: 0.0,
                            sample_rate: sample_rate as f32,
                            fade_step: 1.0 / (sample_rate as f32 * 0.0005),
                            is_playing: is_playing.clone(),
                            frequency_bits: frequency_bits.clone(),
                            use_fundsp: false,
                        };
                        Some(callback)
                    }
                };

                if let Some(callback) = use_fundsp {
                    match AudioStreamBuilder::default()
                        .set_format::<f32>()
                        .set_channel_count::<oboe::Mono>()
                        .set_sample_rate(sample_rate)
                        .set_frames_per_callback(buffer_size)
                        .set_performance_mode(PerformanceMode::PowerSaving)
                        .set_sharing_mode(SharingMode::Exclusive)
                        .set_callback(callback)
                        .open_stream()
                    {
                        Ok(s) => {
                            let actual_frames = s.get_frames_per_callback();
                            let latency_ms = (actual_frames as f32 / sample_rate as f32) * 1000.0;
                            attempt_info.push(format!(
                                "CALLBACK+PowerSaving+Exclusive {}Hz {}→{} frames ({:.2}ms)",
                                sample_rate, buffer_size, actual_frames, latency_ms
                            ));

                            if stream.is_none() || latency_ms < best_latency {
                                best_latency = latency_ms;
                                final_sample_rate = sample_rate as f32;
                                stream = Some(s);
                            }
                        }
                        Err(e) => {
                            attempt_info.push(format!(
                                "CALLBACK+PowerSaving+Exclusive {}Hz {} failed: {}",
                                sample_rate, buffer_size, e
                            ));
                        }
                    }
                }
            }
        }
    }

    // Strategy 3: CALLBACK + Shared mode fallback
    if stream.is_none() {
        println!("Callback exclusive modes failed, trying callback shared mode...");
        for &sample_rate in &sample_rates {
            let use_fundsp = match create_fundsp_synth(sample_rate as f32) {
                Ok(synth) => {
                    let callback = AudioCallback {
                        synth,
                        current_frequency: 440.0,
                        phase: 0.0,
                        envelope: 0.0,
                        sample_rate: sample_rate as f32,
                        fade_step: 1.0 / (sample_rate as f32 * 0.0005),
                        is_playing: is_playing.clone(),
                        frequency_bits: frequency_bits.clone(),
                        use_fundsp: true,
                    };
                    Some(callback)
                }
                Err(_) => {
                    let callback = AudioCallback {
                        synth: sine_hz(440.0), // Dummy value
                        current_frequency: 440.0,
                        phase: 0.0,
                        envelope: 0.0,
                        sample_rate: sample_rate as f32,
                        fade_step: 1.0 / (sample_rate as f32 * 0.0005),
                        is_playing: is_playing.clone(),
                        frequency_bits: frequency_bits.clone(),
                        use_fundsp: false,
                    };
                    Some(callback)
                }
            };

            if let Some(callback) = use_fundsp {
                match AudioStreamBuilder::default()
                    .set_format::<f32>()
                    .set_channel_count::<oboe::Mono>()
                    .set_sample_rate(sample_rate)
                    .set_frames_per_callback(64)
                    .set_performance_mode(PerformanceMode::LowLatency)
                    .set_sharing_mode(SharingMode::Shared)
                    .set_callback(callback)
                    .open_stream()
                {
                    Ok(s) => {
                        let actual_frames = s.get_frames_per_callback();
                        let latency_ms = (actual_frames as f32 / sample_rate as f32) * 1000.0;
                        attempt_info.push(format!(
                            "CALLBACK+LowLatency+Shared {}Hz 64→{} frames ({:.2}ms) - SUCCESS",
                            sample_rate, actual_frames, latency_ms
                        ));

                        if stream.is_none() || latency_ms < best_latency {
                            best_latency = latency_ms;
                            final_sample_rate = sample_rate as f32;
                            stream = Some(s);
                            break;
                        }
                    }
                    Err(e) => {
                        attempt_info.push(format!(
                            "CALLBACK+LowLatency+Shared {}Hz failed: {}",
                            sample_rate, e
                        ));
                    }
                }
            }
        }
    }

    // Get the final stream
    let mut stream = match stream {
        Some(s) => s,
        None => {
            println!("❌ Failed to create any callback audio stream");
            return Err("Failed to initialize callback audio stream".into());
        }
    };

    // Get actual stream parameters for callback mode
    let actual_sample_rate = stream.get_sample_rate() as f32;
    let actual_callback_size = stream.get_frames_per_callback() as usize;

    println!("� CALLBACK MODE Multi-Strategy Results:");
    for info in attempt_info {
        println!("   {}", info);
    }
    println!();

    println!(
        "🎯 Oboe CALLBACK stream: {} Hz, {} frames per callback",
        actual_sample_rate as i32, actual_callback_size
    );

    // Analyze callback buffer quality
    let callback_latency_ms = (actual_callback_size as f32 / actual_sample_rate) * 1000.0;
    if actual_callback_size <= 32 {
        println!(
            "🚀 ULTRA-LOW: Hardware-level latency achieved ({:.2}ms)",
            callback_latency_ms
        );
    } else if actual_callback_size <= 64 {
        println!(
            "🎯 EXCELLENT: Ultra-low callback latency ({:.2}ms)",
            callback_latency_ms
        );
    } else if actual_callback_size <= 128 {
        println!(
            "✅ GOOD: Low callback latency ({:.2}ms)",
            callback_latency_ms
        );
    } else {
        println!(
            "⚠️  Large callback buffer: {} frames ({:.2}ms)",
            actual_callback_size, callback_latency_ms
        );
        println!("💡 Callback mode may still be faster than sync mode");
    }

    // Start the stream
    stream.start()?;
    println!("🔥 Android CALLBACK audio stream started");

    // Audio Performance Metrics for callback mode
    println!("📊 CALLBACK Mode Audio Performance:");
    println!("   Sample Rate: {} Hz", actual_sample_rate as i32);
    println!(
        "   Callback Buffer: {} frames ({:.2}ms)",
        actual_callback_size, callback_latency_ms
    );
    println!("   Attack Time: 0.5ms (ultra-fast)");
    println!(
        "   Expected Touch-to-Sound: ~{:.1}ms (callback mode)",
        callback_latency_ms + 1.0
    );

    println!("🚀 Android CALLBACK audio engine initialized with FunDSP - Zero-copy audio pipeline");

    // Keep stream alive in a static context - callback mode requires this
    // We need to prevent the stream from being dropped
    std::thread::spawn(move || {
        println!("🔧 Callback mode stream keeper thread started");

        // Keep the stream alive by holding a reference to it
        // The callback will continue to be called as long as the stream exists
        loop {
            // Check stream state periodically
            match stream.get_state() {
                oboe::StreamState::Started => {
                    // Stream is running correctly
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
                oboe::StreamState::Paused => {
                    println!("⚠️ Stream paused, attempting to restart...");
                    if let Err(e) = stream.start() {
                        println!("❌ Failed to restart stream: {}", e);
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                oboe::StreamState::Stopped => {
                    println!("⚠️ Stream stopped, attempting to restart...");
                    if let Err(e) = stream.start() {
                        println!("❌ Failed to restart stream: {}", e);
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                state => {
                    println!("🔍 Stream state: {:?}", state);
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        }
    });

    Ok(())
}
