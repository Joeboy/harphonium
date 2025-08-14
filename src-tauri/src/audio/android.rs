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

    // Create callback handler; never block in RT thread
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
            // Generate audio using FunDSP synthesis without locking if unavailable
            match self.synth.try_lock() {
                Ok(mut synth_guard) => {
                    for sample in frames.iter_mut() {
                        *sample = synth_guard.get_sample();
                    }
                }
                Err(_) => {
                    // Fill with silence on contention to avoid glitches / priority inversion
                    frames.fill(0.0);
                }
            }
            DataCallbackResult::Continue
        }
    }

    let callback = AudioCallback {
        synth: synth.clone(),
    };

    println!("🚀 Android audio using FunDSP synthesis (Shared mode)");
    let mut stream = AudioStreamBuilder::default()
        .set_format::<f32>()
        .set_channel_count::<oboe::Mono>()
        .set_sample_rate(24000)
        .set_frames_per_callback(64)
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared)
        .set_callback(AudioCallback {
            synth: synth.clone(),
        })
        .open_stream()?;

    let actual_sample_rate = stream.get_sample_rate() as f32;
    let actual_callback_size = stream.get_frames_per_callback();

    // Align backend sample rate to device stream
    if let Ok(mut s) = synth.lock() {
        s.set_sample_rate(actual_sample_rate);
    }

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
