// Cross-platform audio module for Harphonium synthesizer
use std::sync::{Arc, Mutex};

// Shared synthesis module using FunDSP
mod synthesis;
use synthesis::{FunDSPSynth, Waveform};

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

    pub fn set_master_volume(&self, volume: f32) -> Result<(), String> {
        if let Ok(mut synth) = self.synth.lock() {
            synth.set_master_volume(volume);
            Ok(())
        } else {
            Err("Failed to acquire synth lock".to_string())
        }
    }

    pub fn get_master_volume(&self) -> f32 {
        if let Ok(synth) = self.synth.lock() {
            synth.get_master_volume()
        } else {
            0.7 // Default volume
        }
    }

    /// Set the current waveform
    pub fn set_waveform(&self, waveform: Waveform) -> Result<(), String> {
        if let Ok(mut synth) = self.synth.lock() {
            synth.set_waveform(waveform);
            Ok(())
        } else {
            Err("Failed to acquire synth lock".to_string())
        }
    }

    /// Get the current waveform
    pub fn get_waveform(&self) -> Waveform {
        if let Ok(synth) = self.synth.lock() {
            synth.get_waveform()
        } else {
            Waveform::default()
        }
    }

    /// Set ADSR attack time
    pub fn set_attack(&self, attack: f32) -> Result<(), String> {
        if let Ok(mut synth) = self.synth.lock() {
            synth.set_attack(attack);
            Ok(())
        } else {
            Err("Failed to acquire synth lock".to_string())
        }
    }

    /// Get ADSR attack time
    pub fn get_attack(&self) -> f32 {
        if let Ok(synth) = self.synth.lock() {
            synth.get_attack()
        } else {
            0.02 // Default attack
        }
    }

    /// Set ADSR decay time
    pub fn set_decay(&self, decay: f32) -> Result<(), String> {
        if let Ok(mut synth) = self.synth.lock() {
            synth.set_decay(decay);
            Ok(())
        } else {
            Err("Failed to acquire synth lock".to_string())
        }
    }

    /// Get ADSR decay time
    pub fn get_decay(&self) -> f32 {
        if let Ok(synth) = self.synth.lock() {
            synth.get_decay()
        } else {
            0.2 // Default decay
        }
    }

    /// Set ADSR sustain level
    pub fn set_sustain(&self, sustain: f32) -> Result<(), String> {
        if let Ok(mut synth) = self.synth.lock() {
            synth.set_sustain(sustain);
            Ok(())
        } else {
            Err("Failed to acquire synth lock".to_string())
        }
    }

    /// Get ADSR sustain level
    pub fn get_sustain(&self) -> f32 {
        if let Ok(synth) = self.synth.lock() {
            synth.get_sustain()
        } else {
            0.6 // Default sustain
        }
    }

    /// Set ADSR release time
    pub fn set_release(&self, release: f32) -> Result<(), String> {
        if let Ok(mut synth) = self.synth.lock() {
            synth.set_release(release);
            Ok(())
        } else {
            Err("Failed to acquire synth lock".to_string())
        }
    }

    /// Get ADSR release time
    pub fn get_release(&self) -> f32 {
        if let Ok(synth) = self.synth.lock() {
            synth.get_release()
        } else {
            0.3 // Default release
        }
    }

    pub fn set_delay_time(&self, delay_time: f32) -> Result<(), String> {
        if let Ok(mut synth) = self.synth.lock() {
            synth.set_delay_time(delay_time);
            Ok(())
        } else {
            Err("Failed to acquire synth lock".to_string())
        }
    }

    pub fn get_delay_time(&self) -> f32 {
        if let Ok(synth) = self.synth.lock() {
            synth.get_delay_time()
        } else {
            0.5
        }
    }

    pub fn set_delay_feedback(&self, delay_feedback: f32) -> Result<(), String> {
        if let Ok(mut synth) = self.synth.lock() {
            synth.set_delay_feedback(delay_feedback);
            Ok(())
        } else {
            Err("Failed to acquire synth lock".to_string())
        }
    }

    pub fn get_delay_feedback(&self) -> f32 {
        if let Ok(synth) = self.synth.lock() {
            synth.get_delay_feedback()
        } else {
            0.4 // Default delay feedback
        }
    }

    pub fn set_delay_mix(&self, delay_mix: f32) -> Result<(), String> {
        if let Ok(mut synth) = self.synth.lock() {
            synth.set_delay_mix(delay_mix);
            Ok(())
        } else {
            Err("Failed to acquire synth lock".to_string())
        }
    }

    pub fn get_delay_mix(&self) -> f32 {
        if let Ok(synth) = self.synth.lock() {
            synth.get_delay_mix()
        } else {
            0.8 // Default delay mix
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

pub fn set_master_volume(volume: f32) -> Result<(), String> {
    // Initialize audio if not already done
    if let Err(e) = initialize_audio() {
        return Err(format!("Failed to initialize audio: {}", e));
    }

    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.set_master_volume(volume)
    } else {
        Err("Audio engine not initialized".to_string())
    }
}

pub fn get_master_volume() -> f32 {
    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.get_master_volume()
    } else {
        0.7 // Default volume
    }
}

/// Set the current waveform
pub fn set_waveform(waveform_str: &str) -> Result<(), String> {
    // Initialize audio if not already done
    if let Err(e) = initialize_audio() {
        return Err(format!("Failed to initialize audio: {}", e));
    }

    // Convert string to Waveform enum
    let waveform = match Waveform::from_str(waveform_str) {
        Some(w) => w,
        None => return Err(format!("Invalid waveform: {}", waveform_str)),
    };

    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.set_waveform(waveform)
    } else {
        Err("Audio engine not initialized".to_string())
    }
}

/// Get the current waveform as a string
pub fn get_waveform() -> String {
    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.get_waveform().as_str().to_string()
    } else {
        Waveform::default().as_str().to_string()
    }
}

/// Set ADSR attack time
pub fn set_attack(attack: f32) -> Result<(), String> {
    // Initialize audio if not already done
    if let Err(e) = initialize_audio() {
        return Err(format!("Failed to initialize audio: {}", e));
    }

    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.set_attack(attack)
    } else {
        Err("Audio engine not initialized".to_string())
    }
}

/// Get ADSR attack time
pub fn get_attack() -> f32 {
    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.get_attack()
    } else {
        0.02 // Default attack
    }
}

/// Set ADSR decay time
pub fn set_decay(decay: f32) -> Result<(), String> {
    // Initialize audio if not already done
    if let Err(e) = initialize_audio() {
        return Err(format!("Failed to initialize audio: {}", e));
    }

    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.set_decay(decay)
    } else {
        Err("Audio engine not initialized".to_string())
    }
}

/// Get ADSR decay time
pub fn get_decay() -> f32 {
    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.get_decay()
    } else {
        0.2 // Default decay
    }
}

/// Set ADSR sustain level
pub fn set_sustain(sustain: f32) -> Result<(), String> {
    // Initialize audio if not already done
    if let Err(e) = initialize_audio() {
        return Err(format!("Failed to initialize audio: {}", e));
    }

    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.set_sustain(sustain)
    } else {
        Err("Audio engine not initialized".to_string())
    }
}

/// Get ADSR sustain level
pub fn get_sustain() -> f32 {
    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.get_sustain()
    } else {
        0.6 // Default sustain
    }
}

/// Set ADSR release time
pub fn set_release(release: f32) -> Result<(), String> {
    // Initialize audio if not already done
    if let Err(e) = initialize_audio() {
        return Err(format!("Failed to initialize audio: {}", e));
    }

    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.set_release(release)
    } else {
        Err("Audio engine not initialized".to_string())
    }
}

/// Get ADSR release time
pub fn get_release() -> f32 {
    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.get_release()
    } else {
        0.3 // Default release
    }
}

// Set delay time (in seconds or ms as appropriate for your synth)
pub fn set_delay_time(_delay_time: f32) -> Result<(), String> {
    // Initialize audio if not already done
    if let Err(e) = initialize_audio() {
        return Err(format!("Failed to initialize audio: {}", e));
    }

    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.set_delay_time(_delay_time)
    } else {
        Err("Audio engine not initialized".to_string())
    }
}

pub fn get_delay_time() -> f32 {
    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.get_delay_time()
    } else {
        0.5 // Default delay time
    }
}

pub fn set_delay_feedback(_delay_feedback: f32) -> Result<(), String> {
    // Initialize audio if not already done
    if let Err(e) = initialize_audio() {
        return Err(format!("Failed to initialize audio: {}", e));
    }

    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.set_delay_feedback(_delay_feedback)
    } else {
        Err("Audio engine not initialized".to_string())
    }
}

pub fn get_delay_feedback() -> f32 {
    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.get_delay_feedback()
    } else {
        0.4 // Default delay feedback
    }
}
// Set delay mix (0.0 = dry, 1.0 = fully wet)
pub fn set_delay_mix(_delay_mix: f32) -> Result<(), String> {
    // TODO: Implement actual delay mix control in your synth
    // Initialize audio if not already done
    if let Err(e) = initialize_audio() {
        return Err(format!("Failed to initialize audio: {}", e));
    }

    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.set_delay_mix(_delay_mix)
    } else {
        Err("Audio engine not initialized".to_string())
    }
}

pub fn get_delay_mix() -> f32 {
    if let Some(engine) = AUDIO_ENGINE.get() {
        engine.get_delay_mix()
    } else {
        0.8 // Default delay mix
    }
}
