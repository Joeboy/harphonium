// Desktop audio implementation using cpal with FunDSP integration
use super::synthesis::FunDSPSynth;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub fn start_audio_stream(
    synth: Arc<Mutex<FunDSPSynth>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("No output device available")?;

    let config = device.default_output_config()?;
    let config: cpal::StreamConfig = config.into();

    let sample_rate = config.sample_rate.0 as f32;
    println!(
        "🎵 Desktop audio: {} Hz, {} channels",
        sample_rate, config.channels
    );
    println!("🚀 Desktop audio using FunDSP synthesis (no fallback)");

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // Fill buffer with FunDSP samples
            if let Ok(mut synth_guard) = synth.lock() {
                for frame in data.chunks_mut(config.channels as usize) {
                    let sample = synth_guard.get_sample();
                    for channel_sample in frame.iter_mut() {
                        *channel_sample = sample;
                    }
                }
            }
        },
        |err| eprintln!("Desktop audio stream error: {}", err),
        None,
    )?;

    stream.play()?;

    println!("🎯 Desktop audio stream started");

    // Keep the stream alive by leaking it (in production, you'd want proper lifecycle management)
    std::mem::forget(stream);

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
