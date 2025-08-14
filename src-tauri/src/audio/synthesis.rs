// Shared audio synthesis module using FunDSP
// This module provides FunDSP audio generation for both desktop and Android platforms

use fundsp::hacker::*;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

/// FunDSP-based synthesizer that can be shared across platforms
pub struct FunDSPSynth {
    /// FunDSP synthesis chain with delay effect and ADSR envelope
    synth: Box<dyn AudioUnit + Send>,
    /// Frequency control for the oscillator
    frequency_var: shared::Shared,
    /// Key down state control (0.0 = key up/silent, 1.0 = key down/playing) - used as ADSR gate
    key_down_var: shared::Shared,
    /// Sample rate for proper delay calculation
    sample_rate: f32,
    /// Whether FunDSP is enabled (can be disabled if panics occur)
    enabled: bool,
}

impl FunDSPSynth {
    pub fn new(sample_rate: f32) -> Result<Self, Box<dyn std::error::Error>> {
        // Create shared variables for frequency and key down state
        let frequency_var = shared(440.0);
        let key_down_var = shared(0.0); // 0.0 = key up/silent, 1.0 = key down/playing

        // ADSR parameters: Attack=0.1s, Decay=0.2s, Sustain=0.6, Release=0.3s
        let adsr_envelope = adsr_live(0.02, 0.2, 0.6, 0.3);

        // Synthesis pipeline with ADSR envelope:
        // 1. Generate sine wave
        // 2. Apply ADSR envelope (controlled by key_down_var as gate)
        // 3. Add delay effect
        // 4. Apply final gain/volume control
        let mut synth = Box::new(
            var(&frequency_var)
                >> sine()
                >> (pass() * (var(&key_down_var) >> adsr_envelope))
                >> split()
                >> (pass() + delay(0.3) * 0.3)
                >> join()
                >> mul(0.4), // Final gain
        );

        // Set the correct sample rate for the synthesizer
        synth.set_sample_rate(sample_rate as f64);
        synth.reset();

        println!("🎵 FunDSP initialized at {} Hz sample rate", sample_rate);

        Ok(FunDSPSynth {
            synth,
            frequency_var,
            key_down_var,
            sample_rate,
            enabled: true,
        })
    }

    /// Generate a single mono sample
    pub fn get_sample(&mut self) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        // Use panic catching to handle FunDSP issues gracefully
        if let Ok(result) =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.synth.get_mono()))
        {
            let output = result as f32;
            // Safety: ensure output is finite and in valid range
            if output.is_finite() && output.abs() <= 1.0 {
                output
            } else {
                0.0 // Safety fallback
            }
        } else {
            // If FunDSP panics, disable it for this instance
            self.enabled = false;
            0.0
        }
    }

    /// Generate a single sample with atomic controls (for Android callback)
    pub fn get_sample_from_atomics(
        &mut self,
        key_down: Arc<AtomicBool>,
        frequency_bits: Arc<AtomicU32>,
    ) -> f32 {
        let currently_key_down = key_down.load(Ordering::Relaxed);

        // Update key down state - convert boolean to float for ADSR gate
        let gate_value = if currently_key_down { 1.0 } else { 0.0 };
        self.key_down_var.set_value(gate_value);

        let frequency_bits_val = frequency_bits.load(Ordering::Relaxed);
        let frequency = f32::from_bits(frequency_bits_val);

        // Update frequency directly (UI controls the frequency changes)
        if self.enabled {
            self.frequency_var.set_value(frequency);
        }

        self.get_sample()
    }

    /// Play a note at the specified frequency
    pub fn play_note(&mut self, frequency: f32) {
        if self.enabled {
            self.frequency_var.set_value(frequency);
            self.key_down_var.set_value(1.0); // Gate on - triggers ADSR attack
        }
        
        println!("Playing frequency: {} Hz", frequency);
    }

    /// Stop the current note
    pub fn stop_note(&mut self) {
        if self.enabled {
            self.key_down_var.set_value(0.0); // Gate off - triggers ADSR release
        }
        
        println!("Stopping audio");
    }

    /// Get current playing state by checking the ADSR gate
    pub fn is_playing(&self) -> bool {
        // The ADSR gate value indicates if we're currently in attack/sustain phase
        // 1.0 = gate is on (playing), 0.0 = gate is off (releasing or silent)
        self.key_down_var.value() > 0.0
    }

    /// Get current frequency from the actual synthesis variable
    pub fn get_frequency(&self) -> f32 {
        self.frequency_var.value()
    }
}
