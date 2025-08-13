// Desktop audio implementation using cpal with FunDSP integration
use super::synthesis::FunDSPSynth;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::Arc;

pub fn initialize_audio_engine(
    is_playing: Arc<AtomicBool>,
    frequency_bits: Arc<AtomicU32>,
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

    // Create FunDSP synthesizer (error if FunDSP fails)
    let mut synth = FunDSPSynth::new(sample_rate)?;
    println!("🚀 Desktop audio using FunDSP synthesis (no fallback)");

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // Fill buffer with FunDSP samples (playing state handled in pipeline)
            for frame in data.chunks_mut(config.channels as usize) {
                let sample = synth.get_sample_from_atomics(is_playing.clone(), frequency_bits.clone());
                for channel_sample in frame.iter_mut() {
                    *channel_sample = sample;
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
