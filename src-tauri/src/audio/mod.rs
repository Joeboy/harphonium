// Cross-platform audio module for Harphonium synthesizer
use std::sync::{Arc, Mutex, OnceLock};

// Shared synthesis module using FunDSP
mod synthesis;
use rtrb::Producer;
use synthesis::FunDSPSynth;
pub use synthesis::{AudioEvent, AudioEventResult, Waveform};

// Desktop audio implementation using cpal
#[cfg(not(target_os = "android"))]
mod desktop;

// Android audio implementation using oboe
#[cfg(target_os = "android")]
mod android;

// Cross-platform audio engine wrapper
pub struct AudioEngine {
    synth: Arc<Mutex<FunDSPSynth>>,
}

impl AudioEngine {
    pub fn new(
        event_consumer: rtrb::Consumer<AudioEvent>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Tentative sample rate; platform backends will align it to the device after opening streams
        let sample_rate = 48000.0f32;
        let synth = Arc::new(Mutex::new(FunDSPSynth::new(sample_rate, event_consumer)?));

        let engine = AudioEngine {
            synth: synth.clone(),
        };

        // Initialize the platform-specific audio streaming
        engine.init_platform_audio(synth)?;

        Ok(engine)
    }

    fn init_platform_audio(
        &self,
        synth: Arc<Mutex<FunDSPSynth>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Platform-specific initialization that connects to our synth
        #[cfg(not(target_os = "android"))]
        {
            desktop::start_audio_stream(synth)?;
            println!("Desktop audio stream started");
        }

        #[cfg(target_os = "android")]
        {
            android::start_audio_stream(synth)?;
            println!("Android audio stream started");
        }

        Ok(())
    }

    /// Handle a result immediately, without queuing. Use this for anything
    /// that needs a return value. This locks the audio thread, so is a potential
    /// source of dropouts / glitches. Maybe do something about that at some point
    pub fn handle_event(&self, event: AudioEvent) -> AudioEventResult {
        if let Ok(mut synth) = self.synth.lock() {
            synth.handle_event(event)
        } else {
            AudioEventResult::Err("Failed to acquire synth lock".to_string())
        }
    }
}

// Global audio engine
static AUDIO_ENGINE: OnceLock<AudioEngine> = OnceLock::new();
static EVENT_PRODUCER: OnceLock<Arc<Mutex<Producer<AudioEvent>>>> = OnceLock::new();

pub fn initialize_audio() -> Result<(), Box<dyn std::error::Error>> {
    if AUDIO_ENGINE.get().is_none() {
        let (event_producer, event_consumer) = rtrb::RingBuffer::<AudioEvent>::new(64);

        EVENT_PRODUCER
            .set(Arc::new(Mutex::new(event_producer)))
            .unwrap();

        match AudioEngine::new(event_consumer) {
            Ok(engine) => {
                if AUDIO_ENGINE.set(engine).is_err() {
                    return Err("Failed to initialize audio engine".into());
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// Immediately handle an event, skipping the queue
pub fn handle_audio_event(event: AudioEvent) -> AudioEventResult {
    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.handle_event(event)
    } else {
        AudioEventResult::Err("Audio engine not initialized".to_string())
    }
}

/// Queue an audio event for processing. NB events may be dropped if superceded
/// by subsequent events in the same buffer
pub fn queue_audio_event(event: AudioEvent) -> AudioEventResult {
    if let Some(producer) = EVENT_PRODUCER.get() {
        let mut producer = producer.lock().unwrap();
        match producer.push(event) {
            Ok(_) => AudioEventResult::Ok,
            Err(_) => AudioEventResult::Err("Event queue full".to_string()),
        }
    } else {
        AudioEventResult::Err("Producer not initialized".to_string())
    }
}
