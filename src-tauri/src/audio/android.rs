// Android audio implementation using oboe with FunDSP integration
use super::synthesis::FunDSPSynth;
use std::sync::{Arc, Mutex};

pub fn start_audio_stream(
    synth: Arc<Mutex<FunDSPSynth>>,
) -> Result<(), Box<dyn std::error::Error>> {
    use oboe::{
        AudioOutputCallback, AudioOutputStreamSafe, AudioStream, AudioStreamBase,
        AudioStreamBuilder, AudioStreamSafe, DataCallbackResult, PerformanceMode, SharingMode,
    };

    println!("Initializing Android audio engine with Oboe - CALLBACK MODE");

    // Create callback handler with ultra-low latency processing using FunDSP
    struct AudioCallback {
        synth: Arc<Mutex<FunDSPSynth>>,
    }

    impl AudioOutputCallback for AudioCallback {
        type FrameType = (f32, oboe::Mono); // Correct frame type for mono

        fn on_audio_ready(
            &mut self,
            _stream: &mut dyn AudioOutputStreamSafe,
            frames: &mut [f32],
        ) -> DataCallbackResult {
            // Generate audio using FunDSP synthesis
            if let Ok(mut synth_guard) = self.synth.lock() {
                for sample in frames.iter_mut() {
                    *sample = synth_guard.get_sample();
                }
            }
            DataCallbackResult::Continue
        }
    }

    let callback = AudioCallback {
        synth: synth.clone(),
    };

    // Try different configurations for the best latency
    let buffer_sizes = [16, 24, 32, 48, 64];
    let sample_rates = [48000, 44100];
    let mut stream = None;
    let mut best_latency = f32::MAX;

    println!("🚀 Android audio using FunDSP synthesis (no fallback)");

    // Strategy 1: CALLBACK + LowLatency + Exclusive - most aggressive
    for &sr in &sample_rates {
        for &buffer_size in &buffer_sizes {
            match AudioStreamBuilder::default()
                .set_format::<f32>()
                .set_channel_count::<oboe::Mono>()
                .set_sample_rate(sr)
                .set_frames_per_callback(buffer_size)
                .set_performance_mode(PerformanceMode::LowLatency)
                .set_sharing_mode(SharingMode::Exclusive)
                .set_callback(AudioCallback {
                    synth: synth.clone(),
                })
                .open_stream()
            {
                Ok(s) => {
                    let actual_frames = s.get_frames_per_callback();
                    let latency_ms = (actual_frames as f32 / sr as f32) * 1000.0;

                    if latency_ms < best_latency {
                        best_latency = latency_ms;
                        stream = Some(s);
                    }
                    println!(
                        "🔥 CALLBACK+LowLatency+Exclusive {}Hz {}→{} frames ({:.2}ms)",
                        sr, buffer_size, actual_frames, latency_ms
                    );
                }
                Err(_) => {
                    // Try next configuration
                }
            }
        }
    }

    // Strategy 2: Fallback to shared mode if exclusive failed
    if stream.is_none() {
        println!("Exclusive mode failed, trying shared mode...");
        for &sr in &sample_rates {
            match AudioStreamBuilder::default()
                .set_format::<f32>()
                .set_channel_count::<oboe::Mono>()
                .set_sample_rate(sr)
                .set_frames_per_callback(64)
                .set_performance_mode(PerformanceMode::LowLatency)
                .set_sharing_mode(SharingMode::Shared)
                .set_callback(AudioCallback {
                    synth: synth.clone(),
                })
                .open_stream()
            {
                Ok(s) => {
                    stream = Some(s);
                    break;
                }
                Err(_) => continue,
            }
        }
    }

    // Get the final stream
    let mut stream = match stream {
        Some(s) => s,
        None => {
            return Err("Failed to initialize callback audio stream".into());
        }
    };

    let actual_sample_rate = stream.get_sample_rate() as f32;
    let actual_callback_size = stream.get_frames_per_callback();

    println!(
        "🎯 Oboe CALLBACK stream: {} Hz, {} frames per callback",
        actual_sample_rate as i32, actual_callback_size
    );

    // Start the stream
    stream.start()?;
    println!("🔥 Android CALLBACK audio stream started");

    // Keep stream alive in a background thread
    std::thread::spawn(move || {
        println!("🔧 Callback mode stream keeper thread started");
        loop {
            match stream.get_state() {
                oboe::StreamState::Started => {
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
                oboe::StreamState::Paused => {
                    println!("⚠️ Stream paused, attempting to restart...");
                    let _ = stream.start();
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                oboe::StreamState::Stopped => {
                    println!("⚠️ Stream stopped, attempting to restart...");
                    let _ = stream.start();
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                _ => {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        }
    });

    Ok(())
}

// // Legacy function for backwards compatibility
// pub fn initialize_audio_engine(
//     _is_playing: std::sync::Arc<std::sync::atomic::AtomicBool>,
//     _frequency_bits: std::sync::Arc<std::sync::atomic::AtomicU32>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     eprintln!("Warning: initialize_audio_engine is deprecated. Use start_audio_stream instead.");
//     Ok(())
// }
