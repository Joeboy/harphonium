// Cross-platform audio module for Harphonium synthesizer
use std::sync::{Arc, Mutex};

// Shared synthesis module using FunDSP
mod synthesis;
pub use synthesis::{AudioEvent, AudioEventResult, Waveform};
use synthesis::FunDSPSynth;

// Desktop audio implementation using cpal
#[cfg(not(target_os = "android"))]
mod desktop;

// Android audio implementation using oboe
#[cfg(target_os = "android")]
mod android;

// Cross-platform audio engine wrapper
pub struct AudioEngine {
    synth: Arc<Mutex<FunDSPSynth>>,
    // Platform-specific stream is kept alive internally
}

impl AudioEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Tentative sample rate; platform backends will align it to the device after opening streams
        let sample_rate = 48000.0f32;
        let synth = Arc::new(Mutex::new(FunDSPSynth::new(sample_rate)?));

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

    pub fn handle_event(&self, event: AudioEvent) -> AudioEventResult {
        if let Ok(mut synth) = self.synth.lock() {
            synth.handle_event(event)
        } else {
            AudioEventResult::Err("Failed to acquire synth lock".to_string())
        }
    }
}

// Global audio engine
static AUDIO_ENGINE: std::sync::OnceLock<AudioEngine> = std::sync::OnceLock::new();

pub fn initialize_audio() -> Result<(), Box<dyn std::error::Error>> {
    if AUDIO_ENGINE.get().is_none() {
        match AudioEngine::new() {
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

pub fn handle_audio_event(event: AudioEvent) -> AudioEventResult {
    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.handle_event(event)
    } else {
        AudioEventResult::Err("Audio engine not initialized".to_string())
    }
}
