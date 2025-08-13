// Android audio implementation using oboe
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

pub fn initialize_audio_engine(
    is_playing: Arc<AtomicBool>,
    frequency_bits: Arc<AtomicU32>,
) -> Result<(), Box<dyn std::error::Error>> {
    use oboe::{
        AudioOutputCallback, AudioOutputStreamSafe, AudioStream, AudioStreamBase,
        AudioStreamBuilder, AudioStreamSafe, DataCallbackResult, PerformanceMode, SharingMode,
        StreamState,
    };

    println!("Initializing Android audio engine with Oboe - CALLBACK MODE");

    // Create callback handler with ultra-low latency processing
    struct AudioCallback {
        phase: f32,
        envelope: f32,
        sample_rate: f32,
        fade_step: f32,
        is_playing: Arc<AtomicBool>,
        frequency_bits: Arc<AtomicU32>,
    }

    impl AudioOutputCallback for AudioCallback {
        type FrameType = (f32, oboe::Mono); // Correct frame type for mono

        fn on_audio_ready(
            &mut self,
            _stream: &mut dyn AudioOutputStreamSafe,
            frames: &mut [f32],
        ) -> DataCallbackResult {
            let playing = self.is_playing.load(Ordering::Relaxed);
            let frequency = f32::from_bits(self.frequency_bits.load(Ordering::Relaxed));

            if playing && frequency > 0.0 {
                // Instant attack - fade in quickly
                if self.envelope < 1.0 {
                    self.envelope = (self.envelope + self.fade_step * 4.0).min(1.0);
                    // 4x faster attack
                }

                // Generate sine wave directly in callback for mono
                for sample in frames.iter_mut() {
                    *sample = (self.phase * 2.0 * std::f32::consts::PI).sin() * 0.5 * self.envelope;

                    self.phase += frequency / self.sample_rate;
                    if self.phase >= 1.0 {
                        self.phase -= 1.0;
                    }
                }
            } else {
                // Fade out
                if self.envelope > 0.0 {
                    self.envelope = (self.envelope - self.fade_step).max(0.0);

                    for sample in frames.iter_mut() {
                        if self.envelope > 0.0 {
                            *sample = (self.phase * 2.0 * std::f32::consts::PI).sin()
                                * 0.5
                                * self.envelope;

                            self.phase += frequency / self.sample_rate;
                            if self.phase >= 1.0 {
                                self.phase -= 1.0;
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

    // Strategy 1: CALLBACK + LowLatency + Exclusive - most aggressive
    for &sample_rate in &sample_rates {
        for &buffer_size in &buffer_sizes {
            let callback = AudioCallback {
                phase: 0.0,
                envelope: 0.0,
                sample_rate: sample_rate as f32,
                fade_step: 1.0 / (sample_rate as f32 * 0.0005), // 0.5ms fade (very fast)
                is_playing: is_playing.clone(),
                frequency_bits: frequency_bits.clone(),
            };

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

    // Strategy 2: CALLBACK + PowerSaving + Exclusive
    if stream.is_none() {
        for &sample_rate in &sample_rates {
            for &buffer_size in &buffer_sizes {
                let callback = AudioCallback {
                    phase: 0.0,
                    envelope: 0.0,
                    sample_rate: sample_rate as f32,
                    fade_step: 1.0 / (sample_rate as f32 * 0.0005),
                    is_playing: is_playing.clone(),
                    frequency_bits: frequency_bits.clone(),
                };

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

    // Strategy 3: CALLBACK + Shared mode fallback
    if stream.is_none() {
        println!("Callback exclusive modes failed, trying callback shared mode...");
        for &sample_rate in &sample_rates {
            let callback = AudioCallback {
                phase: 0.0,
                envelope: 0.0,
                sample_rate: sample_rate as f32,
                fade_step: 1.0 / (sample_rate as f32 * 0.0005),
                is_playing: is_playing.clone(),
                frequency_bits: frequency_bits.clone(),
            };

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

    println!("🚀 Android CALLBACK audio engine initialized - Zero-copy audio pipeline");

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
