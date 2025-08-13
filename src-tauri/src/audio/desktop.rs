// Desktop audio implementation using cpal
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
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
    let mut phase = 0.0f32;

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let freq_bits = frequency_bits.load(Ordering::Relaxed);
            let freq = f32::from_bits(freq_bits);
            let playing = is_playing.load(Ordering::Relaxed);

            for sample in data.iter_mut() {
                if playing {
                    // Generate sine wave
                    *sample = (phase * 2.0 * std::f32::consts::PI).sin() * 0.2; // 20% volume
                    phase += freq / sample_rate;
                    if phase >= 1.0 {
                        phase -= 1.0;
                    }
                } else {
                    *sample = 0.0;
                }
            }
        },
        |err| eprintln!("Desktop audio stream error: {}", err),
        None,
    )?;

    stream.play()?;

    // Keep the stream alive by leaking it (in production, you'd want proper lifecycle management)
    std::mem::forget(stream);

    Ok(())
}
