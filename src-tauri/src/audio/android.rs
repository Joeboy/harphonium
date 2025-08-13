// Android audio implementation using oboe
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

pub fn initialize_audio_engine(
    is_playing: Arc<AtomicBool>,
    frequency_bits: Arc<AtomicU32>,
) -> Result<(), Box<dyn std::error::Error>> {
    use oboe::{
        AudioOutputStreamSync, AudioStream, AudioStreamBuilder, PerformanceMode, SharingMode,
    };

    println!("Initializing Android audio engine with Oboe");

    // Create audio stream configuration
    let mut stream = AudioStreamBuilder::default()
        .set_performance_mode(PerformanceMode::LowLatency)
        .set_sharing_mode(SharingMode::Shared) // Use Shared mode for better compatibility
        .set_format::<f32>()
        .set_channel_count::<oboe::Mono>()
        .set_sample_rate(44100)
        .set_buffer_capacity_in_frames(1024)
        .open_stream()?;

    println!("Android audio stream created successfully");

    // Start the stream
    stream.start()?;
    println!("Android audio stream started");

    // Move stream into the thread
    std::thread::spawn(move || {
        let mut phase = 0.0f32;
        let sample_rate = 44100.0f32;
        let buffer_size = 256;
        let mut buffer = vec![0.0f32; buffer_size];

        loop {
            if is_playing.load(Ordering::Relaxed) {
                let freq = f32::from_bits(frequency_bits.load(Ordering::Relaxed));

                // Generate sine wave samples
                for sample in buffer.iter_mut() {
                    *sample = (phase * 2.0 * std::f32::consts::PI).sin() * 0.3; // Volume at 30%
                    phase += freq / sample_rate;
                    if phase >= 1.0 {
                        phase -= 1.0;
                    }
                }

                // Try to write audio data to the stream
                if let Ok(frames_written) = stream.write(&buffer, 0) {
                    // Audio data written successfully
                    if frames_written == 0 {
                        // If no frames were written, wait a bit
                        std::thread::sleep(std::time::Duration::from_millis(1));
                    }
                } else {
                    // Error writing audio, wait and retry
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
            } else {
                // Not playing, fill with silence
                buffer.fill(0.0);
                let _ = stream.write(&buffer, 0);
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    });

    println!("Android audio engine initialized");
    Ok(())
}
