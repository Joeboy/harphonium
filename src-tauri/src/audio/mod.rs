// Cross-platform audio module for SynthMob synthesizer
use std::sync::{Arc, Mutex};

// Shared synthesis module using FunDSP
mod synthesis;
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
        let sample_rate = 48000.0f32; // Default sample rate
        let synth = Arc::new(Mutex::new(FunDSPSynth::new(sample_rate)?));
        
        let engine = AudioEngine {
            synth: synth.clone(),
        };
        
        // Initialize the platform-specific audio streaming
        engine.init_platform_audio(synth)?;
        
        Ok(engine)
    }

    fn init_platform_audio(&self, synth: Arc<Mutex<FunDSPSynth>>) -> Result<(), Box<dyn std::error::Error>> {
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

    pub fn play_note(&self, frequency: f32) -> Result<(), String> {
        if let Ok(mut synth) = self.synth.lock() {
            synth.play_note(frequency);
            Ok(())
        } else {
            Err("Failed to acquire synth lock".to_string())
        }
    }

    pub fn stop_note(&self) -> Result<(), String> {
        if let Ok(mut synth) = self.synth.lock() {
            synth.stop_note();
            Ok(())
        } else {
            Err("Failed to acquire synth lock".to_string())
        }
    }

    pub fn is_playing(&self) -> bool {
        if let Ok(synth) = self.synth.lock() {
            synth.is_playing()
        } else {
            false
        }
    }

    pub fn get_frequency(&self) -> f32 {
        if let Ok(synth) = self.synth.lock() {
            synth.get_frequency()
        } else {
            440.0
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

pub fn play_frequency(frequency: f32) -> Result<(), String> {
    // Initialize audio if not already done
    if let Err(e) = initialize_audio() {
        return Err(format!("Failed to initialize audio: {}", e));
    }

    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.play_note(frequency)
    } else {
        Err("Audio engine not initialized".to_string())
    }
}

pub fn stop_audio() -> Result<(), String> {
    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.stop_note()
    } else {
        Err("Audio engine not initialized".to_string())
    }
}

pub fn is_audio_playing() -> bool {
    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.is_playing()
    } else {
        false
    }
}

pub fn get_current_frequency() -> f32 {
    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.get_frequency()
    } else {
        440.0
    }
}
