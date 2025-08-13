// Desktop audio implementation using cpal with FunDSP integration
use super::synthesis::UnifiedSynth;
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

    // Create unified synthesizer with FunDSP support
    let mut synth = UnifiedSynth::new(sample_rate);

    if synth.is_using_fundsp() {
        println!("🚀 Desktop audio using FunDSP synthesis");
    } else {
        println!("⚡ Desktop audio using legacy synthesis");
    }

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // Fill the buffer using the unified synthesizer
            synth.fill_buffer(data, is_playing.clone(), frequency_bits.clone());
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
