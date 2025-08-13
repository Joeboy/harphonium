// Cross-platform audio module for SynthMob synthesizer
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

// Shared synthesis module using FunDSP
mod synthesis;

// Desktop audio implementation using cpal
#[cfg(not(target_os = "android"))]
mod desktop;

// Android audio implementation using oboe
#[cfg(target_os = "android")]
mod android;

// Cross-platform audio state
pub struct AudioState {
    pub is_playing: Arc<AtomicBool>,
    pub frequency_bits: Arc<AtomicU32>,
}

impl AudioState {
    pub fn new() -> Self {
        AudioState {
            is_playing: Arc::new(AtomicBool::new(false)),
            frequency_bits: Arc::new(AtomicU32::new(440.0f32.to_bits())),
        }
    }

    pub fn play_note(&self, freq: f32) {
        self.frequency_bits.store(freq.to_bits(), Ordering::Relaxed);
        self.is_playing.store(true, Ordering::Relaxed);
        println!("Playing frequency: {} Hz", freq);
    }

    pub fn stop_note(&self) {
        self.is_playing.store(false, Ordering::Relaxed);
        println!("Stopping audio");
    }
}

// Global audio state
static AUDIO_STATE: std::sync::OnceLock<AudioState> = std::sync::OnceLock::new();
static AUDIO_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn initialize_audio() -> Result<(), Box<dyn std::error::Error>> {
    if AUDIO_INITIALIZED.load(Ordering::Relaxed) {
        return Ok(());
    }

    let state = AUDIO_STATE.get_or_init(|| AudioState::new());

    // Platform-specific initialization
    #[cfg(not(target_os = "android"))]
    {
        desktop::initialize_audio_engine(
            Arc::clone(&state.is_playing),
            Arc::clone(&state.frequency_bits),
        )?;
        println!("Desktop audio engine initialized");
    }

    #[cfg(target_os = "android")]
    {
        android::initialize_audio_engine(
            Arc::clone(&state.is_playing),
            Arc::clone(&state.frequency_bits),
        )?;
        println!("Android audio engine initialized");
    }

    AUDIO_INITIALIZED.store(true, Ordering::Relaxed);
    Ok(())
}

pub fn play_frequency(frequency: f32) -> Result<(), String> {
    let state = AUDIO_STATE.get_or_init(|| AudioState::new());

    // Initialize audio if not already done
    if !AUDIO_INITIALIZED.load(Ordering::Relaxed) {
        if let Err(e) = initialize_audio() {
            return Err(format!("Failed to initialize audio: {}", e));
        }
    }

    state.play_note(frequency);
    Ok(())
}

pub fn stop_audio() -> Result<(), String> {
    let state = AUDIO_STATE.get_or_init(|| AudioState::new());
    state.stop_note();
    Ok(())
}
