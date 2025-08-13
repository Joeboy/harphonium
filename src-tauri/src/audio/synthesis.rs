// Shared audio synthesis module using FunDSP
// This module provides FunDSP audio generation for both desktop and Android platforms

use fundsp::hacker::{sine_hz, An, Constant, Pipe, Sine, U1};

/// FunDSP-based synthesizer that can be shared across platforms
pub struct FunDSPSynth {
    /// FunDSP sine oscillator
    synth: An<Pipe<Constant<U1>, Sine>>,
    /// Current frequency for frequency change detection
    current_frequency: f64,
    /// Sample rate for proper oscillator initialization
    sample_rate: f32,
    /// Whether FunDSP is enabled (can be disabled if panics occur)
    enabled: bool,
}

impl FunDSPSynth {
    /// Create a new FunDSP synthesizer
    pub fn new(sample_rate: f32) -> Result<Self, Box<dyn std::error::Error>> {
        let mut synth = sine_hz(440.0);
        synth.set_sample_rate(sample_rate as f64);

        Ok(FunDSPSynth {
            synth,
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
            // Create a new sine oscillator with the updated frequency
            self.synth = sine_hz(frequency);
            self.synth.set_sample_rate(self.sample_rate as f64);
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
                output * 0.5 // Apply volume scaling
            } else {
                0.0 // Safety fallback
            }
        } else {
            // If FunDSP panics, disable it for this instance
            self.enabled = false;
            0.0
        }
    }
}
