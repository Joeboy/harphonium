// Shared audio synthesis module using FunDSP
// This module provides unified audio generation for both desktop and Android platforms

use fundsp::hacker::{sine_hz, An, Constant, Pipe, Sine, U1};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

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

    /// Check if FunDSP is still enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Reset/re-enable the synthesizer (useful after recovering from errors)
    pub fn reset(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.current_frequency = 440.0;
        let mut synth = sine_hz(440.0);
        synth.set_sample_rate(sample_rate as f64);
        self.synth = synth;
        self.enabled = true;
    }
}

/// Legacy sine wave generator for fallback when FunDSP is not available
pub struct LegacySynth {
    phase: f32,
    envelope: f32,
    sample_rate: f32,
    fade_step: f32,
}

impl LegacySynth {
    /// Create a new legacy synthesizer
    pub fn new(sample_rate: f32) -> Self {
        LegacySynth {
            phase: 0.0,
            envelope: 0.0,
            sample_rate,
            fade_step: 1.0 / (sample_rate * 0.0005), // 0.5ms fade (very fast)
        }
    }

    /// Generate a single mono sample
    pub fn get_sample(&mut self, frequency: f32, is_playing: bool) -> f32 {
        if is_playing && frequency > 0.0 {
            // Instant attack - fade in quickly
            if self.envelope < 1.0 {
                self.envelope = (self.envelope + self.fade_step * 4.0).min(1.0);
                // 4x faster attack
            }

            // Generate sine wave
            let sine_val = (self.phase * 2.0 * std::f32::consts::PI).sin();
            let output = if sine_val.is_finite() {
                sine_val * 0.5 * self.envelope
            } else {
                0.0
            };

            self.phase += frequency / self.sample_rate;
            // Ensure phase stays in valid range
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            } else if !self.phase.is_finite() {
                self.phase = 0.0; // Reset if phase becomes invalid
            }

            output
        } else {
            // Fade out
            if self.envelope > 0.0 {
                self.envelope = (self.envelope - self.fade_step).max(0.0);

                if self.envelope > 0.0 {
                    let sine_val = (self.phase * 2.0 * std::f32::consts::PI).sin();
                    let output = if sine_val.is_finite() {
                        sine_val * 0.5 * self.envelope
                    } else {
                        0.0
                    };

                    self.phase += frequency / self.sample_rate;
                    if self.phase >= 1.0 {
                        self.phase -= 1.0;
                    } else if !self.phase.is_finite() {
                        self.phase = 0.0; // Reset if phase becomes invalid
                    }

                    output
                } else {
                    self.phase = 0.0; // Reset phase when silent
                    0.0
                }
            } else {
                // Silence
                self.phase = 0.0;
                0.0
            }
        }
    }
}

/// Unified synthesizer that tries FunDSP first, falls back to legacy
pub struct UnifiedSynth {
    fundsp_synth: Option<FunDSPSynth>,
    legacy_synth: LegacySynth,
    sample_rate: f32,
}

impl UnifiedSynth {
    /// Create a new unified synthesizer
    pub fn new(sample_rate: f32) -> Self {
        // Try to create FunDSP synthesizer
        let fundsp_synth = match FunDSPSynth::new(sample_rate) {
            Ok(synth) => {
                println!("🚀 FunDSP synthesizer initialized");
                Some(synth)
            }
            Err(e) => {
                println!("⚠️ FunDSP init failed: {}, using fallback", e);
                None
            }
        };

        let legacy_synth = LegacySynth::new(sample_rate);

        UnifiedSynth {
            fundsp_synth,
            legacy_synth,
            sample_rate,
        }
    }

    /// Fill an audio buffer with synthesized samples
    pub fn fill_buffer(
        &mut self,
        buffer: &mut [f32],
        is_playing: Arc<AtomicBool>,
        frequency_bits: Arc<AtomicU32>,
    ) {
        let playing = is_playing.load(Ordering::Relaxed);
        let raw_frequency = f32::from_bits(frequency_bits.load(Ordering::Relaxed));

        // CRITICAL: Validate frequency to prevent audio callback panics
        let frequency =
            if raw_frequency.is_finite() && raw_frequency > 0.0 && raw_frequency < 20000.0 {
                raw_frequency
            } else {
                0.0 // Safe fallback
            };

        // Try FunDSP first if available and enabled
        if let Some(ref mut fundsp_synth) = self.fundsp_synth {
            if fundsp_synth.is_enabled() && playing && frequency > 0.0 {
                // Update frequency
                fundsp_synth.set_frequency(frequency);

                // Generate audio using FunDSP
                for sample in buffer.iter_mut() {
                    *sample = fundsp_synth.get_sample();
                }
                return;
            }

            // If FunDSP is disabled, fall back to legacy
            if !fundsp_synth.is_enabled() {
                println!("⚠️ FunDSP disabled, falling back to legacy synthesis");
                self.fundsp_synth = None; // Remove disabled FunDSP instance
            }
        }

        // Use legacy synthesis
        for sample in buffer.iter_mut() {
            *sample = self.legacy_synth.get_sample(frequency, playing);
        }
    }

    /// Generate a single sample (for callback-style audio)
    pub fn get_sample(
        &mut self,
        is_playing: Arc<AtomicBool>,
        frequency_bits: Arc<AtomicU32>,
    ) -> f32 {
        let playing = is_playing.load(Ordering::Relaxed);
        let raw_frequency = f32::from_bits(frequency_bits.load(Ordering::Relaxed));

        // CRITICAL: Validate frequency to prevent audio callback panics
        let frequency =
            if raw_frequency.is_finite() && raw_frequency > 0.0 && raw_frequency < 20000.0 {
                raw_frequency
            } else {
                0.0 // Safe fallback
            };

        // Try FunDSP first if available and enabled
        if let Some(ref mut fundsp_synth) = self.fundsp_synth {
            if fundsp_synth.is_enabled() && playing && frequency > 0.0 {
                // Update frequency
                fundsp_synth.set_frequency(frequency);
                return fundsp_synth.get_sample();
            }

            // If FunDSP is disabled, fall back to legacy
            if !fundsp_synth.is_enabled() {
                println!("⚠️ FunDSP disabled, falling back to legacy synthesis");
                self.fundsp_synth = None; // Remove disabled FunDSP instance
            }
        }

        // Use legacy synthesis
        self.legacy_synth.get_sample(frequency, playing)
    }

    /// Check if using FunDSP
    pub fn is_using_fundsp(&self) -> bool {
        self.fundsp_synth
            .as_ref()
            .map(|s| s.is_enabled())
            .unwrap_or(false)
    }

    /// Get current sample rate
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }
}
