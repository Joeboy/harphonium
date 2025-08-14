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
    /// Playing state control (0.0 = silent, 1.0 = playing) - also used as ADSR gate
    playing_var: shared::Shared,
    /// Current frequency for frequency change detection
    current_frequency: f64,
    /// Sample rate for proper delay calculation
    sample_rate: f32,
    /// Whether FunDSP is enabled (can be disabled if panics occur)
    enabled: bool,
}

impl FunDSPSynth {
    pub fn new(sample_rate: f32) -> Result<Self, Box<dyn std::error::Error>> {
        // Create shared variables for frequency and playing state
        let frequency_var = shared(440.0);
        let playing_var = shared(0.0); // 0.0 = silent, 1.0 = playing (also used as ADSR gate)

        // ADSR parameters: Attack=0.1s, Decay=0.2s, Sustain=0.6, Release=0.3s
        let adsr_envelope = adsr_live(0.02, 0.2, 0.6, 0.3);

        // Synthesis pipeline with ADSR envelope:
        // 1. Generate sine wave
        // 2. Apply ADSR envelope (controlled by playing_var as gate)
        // 3. Add delay effect
        let mut synth = Box::new(
            var(&frequency_var)
                >> sine()
                >> (pass() * (var(&playing_var) >> adsr_envelope))
                >> split()
                >> (pass() + delay(0.5) * 0.3)
                >> join(),
        );

        // Set the correct sample rate for the synthesizer
        synth.set_sample_rate(sample_rate as f64);
        synth.reset();

        println!("🎵 FunDSP initialized at {} Hz sample rate", sample_rate);

        Ok(FunDSPSynth {
            synth,
            frequency_var,
            playing_var,
            current_frequency: 440.0,
            sample_rate,
            enabled: true,
        })
    }

    /// Update the frequency if it has changed
    pub fn set_frequency(&mut self, frequency: f32) {
        if !self.enabled {
            return;
        }

        let new_freq = frequency as f64;
        if (new_freq - self.current_frequency).abs() > 0.1 {
            self.current_frequency = new_freq;
            self.frequency_var.set_value(frequency as f32);
        }
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
                output * 0.4 // Apply volume scaling (reduced for delay effect)
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
        let currently_playing = key_down.load(Ordering::Relaxed);

        // Update playing state - this now directly controls both audio gating and ADSR envelope
        let playing = if currently_playing { 1.0 } else { 0.0 };
        self.playing_var.set_value(playing);

        let frequency_bits_val = frequency_bits.load(Ordering::Relaxed);
        let frequency = f32::from_bits(frequency_bits_val);

        // Update frequency if needed
        self.set_frequency(frequency);

        self.get_sample()
    }
}
