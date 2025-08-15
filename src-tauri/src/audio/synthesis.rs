// Shared audio synthesis module using FunDSP
// This module provides FunDSP audio generation for both desktop and Android platforms

use fundsp::hacker::*;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

/// Waveform types available in the synthesizer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Waveform {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

impl Default for Waveform {
    fn default() -> Self {
        Waveform::Sine
    }
}

impl Waveform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Waveform::Sine => "sine",
            Waveform::Square => "square",
            Waveform::Sawtooth => "sawtooth",
            Waveform::Triangle => "triangle",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sine" => Some(Waveform::Sine),
            "square" => Some(Waveform::Square),
            "sawtooth" => Some(Waveform::Sawtooth),
            "triangle" => Some(Waveform::Triangle),
            _ => None,
        }
    }

    /// Create the appropriate oscillator for this waveform
    fn create_oscillator(&self) -> Box<dyn AudioUnit + Send> {
        match self {
            Waveform::Sine => Box::new(sine()),
            Waveform::Square => Box::new(square()),
            Waveform::Sawtooth => Box::new(saw()),
            Waveform::Triangle => Box::new(triangle()),
        }
    }
}

/// FunDSP-based synthesizer that can be shared across platforms
pub struct FunDSPSynth {
    /// FunDSP Net frontend for dynamic modifications
    net: Net,
    /// FunDSP backend for audio processing
    backend: Box<dyn AudioUnit + Send>,
    /// ID of the oscillator node in the net
    oscillator_id: NodeId,
    /// Current waveform selection
    current_waveform: Waveform,
    /// Frequency control for the oscillator
    frequency_var: shared::Shared,
    /// Key down state control (0.0 = key up/silent, 1.0 = key down/playing) - used as ADSR gate
    key_down_var: shared::Shared,
    /// Master volume control (0.0 = silent, 1.0 = full volume)
    master_volume_var: shared::Shared,
    /// ADSR envelope parameters
    attack_var: shared::Shared,
    decay_var: shared::Shared,
    sustain_var: shared::Shared,
    release_var: shared::Shared,
    /// Sample rate for proper delay calculation
    sample_rate: f32,
    /// Whether FunDSP is enabled (can be disabled if panics occur)
    enabled: bool,
}

impl FunDSPSynth {
    pub fn new(sample_rate: f32) -> Result<Self, Box<dyn std::error::Error>> {
        let frequency_var = shared(440.0);
        let key_down_var = shared(0.0); // 0.0 = key up/silent, 1.0 = key down/playing
        let master_volume_var = shared(0.7); // Default to 70% volume

        // ADSR envelope parameters with reasonable defaults
        let attack_var = shared(0.02); // 20ms attack
        let decay_var = shared(0.2); // 200ms decay
        let sustain_var = shared(0.6); // 60% sustain level
        let release_var = shared(0.3); // 300ms release

        // This adsr envelope should be fed by the attack / decay / sustain / release values, but I haven't figured out how to do that
        let adsr_envelope2 = adsr_live(0.05, 0.2, 0.6, 0.3);
        let mut net = Net::new(0, 1);

        // Create the synthesis chain dynamically
        let freq_dc_id = net.push(Box::new(var(&frequency_var)));

        // Start with sine wave as default
        let current_waveform = Waveform::default();
        let oscillator_id = net.push(current_waveform.create_oscillator());
        net.pipe_all(freq_dc_id, oscillator_id);

        let adsr_id = net.push(Box::new(pass() * (var(&key_down_var) >> adsr_envelope2)));
        net.pipe_all(oscillator_id, adsr_id);

        let tail = split()
            >> (pass() + delay(0.3) * 0.3)
            >> join()
            >> mul(0.4)
            >> (pass() * var(&master_volume_var));

        let tail_id = net.push(Box::new(tail));
        net.pipe_all(adsr_id, tail_id);

        net.pipe_output(tail_id);

        let mut backend = net.backend();
        backend.set_sample_rate(sample_rate as f64);
        backend.reset();

        println!(
            "🎵 FunDSP initialized at {} Hz sample rate with {} waveform",
            sample_rate,
            current_waveform.as_str()
        );

        Ok(FunDSPSynth {
            net,
            backend: Box::new(backend),
            oscillator_id,
            current_waveform,
            frequency_var,
            key_down_var,
            master_volume_var,
            attack_var,
            decay_var,
            sustain_var,
            release_var,
            sample_rate,
            enabled: true,
        })
    }

    // Lock-free control handles (cheap clones)
    pub fn frequency_handle(&self) -> shared::Shared {
        self.frequency_var.clone()
    }

    pub fn gate_handle(&self) -> shared::Shared {
        self.key_down_var.clone()
    }

    pub fn volume_handle(&self) -> shared::Shared {
        self.master_volume_var.clone()
    }

    /// Update the backend sample rate and reset safely.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        if sample_rate > 0.0 {
            self.sample_rate = sample_rate;
            self.backend.set_sample_rate(sample_rate as f64);
            self.backend.reset();
        }
    }

    /// Switch to a new waveform using dynamic Net replacement
    pub fn set_waveform(&mut self, new_waveform: Waveform) {
        if new_waveform == self.current_waveform || !self.enabled {
            return; // No change needed
        }

        // Replace the oscillator node with the new waveform
        self.net
            .replace(self.oscillator_id, new_waveform.create_oscillator());

        // Commit the changes to the backend
        self.net.commit();

        self.current_waveform = new_waveform;

        println!(
            "🔄 Switched to {} waveform using Net.replace()",
            new_waveform.as_str()
        );
    }

    /// Get the current waveform
    pub fn get_waveform(&self) -> Waveform {
        self.current_waveform
    }

    /// Generate a single mono sample
    pub fn get_sample(&mut self) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        // Fast path: direct sample fetch and clamp
        let output = self.backend.get_mono();
        if output.is_finite() {
            output.clamp(-1.0, 1.0)
        } else {
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

    /// Set master volume (0.0 = silent, 1.0 = full volume)
    pub fn set_master_volume(&mut self, volume: f32) {
        // Clamp volume to valid range
        let clamped_volume = volume.clamp(0.0, 1.0);

        if self.enabled {
            self.master_volume_var.set_value(clamped_volume);
        }
    }

    /// Get current master volume
    pub fn get_master_volume(&self) -> f32 {
        self.master_volume_var.value()
    }

    /// Set ADSR attack time (in seconds)
    pub fn set_attack(&mut self, attack: f32) {
        println!("Setting attack to {}", attack);
        let clamped_attack = attack.clamp(0.001, 5.0); // 1ms to 5s
        self.attack_var.set_value(clamped_attack);
    }

    /// Get ADSR attack time
    pub fn get_attack(&self) -> f32 {
        self.attack_var.value()
    }

    /// Set ADSR decay time (in seconds)
    pub fn set_decay(&mut self, decay: f32) {
        let clamped_decay = decay.clamp(0.001, 5.0); // 1ms to 5s
        self.decay_var.set_value(clamped_decay);
    }

    /// Get ADSR decay time
    pub fn get_decay(&self) -> f32 {
        self.decay_var.value()
    }

    /// Set ADSR sustain level (0.0 to 1.0)
    pub fn set_sustain(&mut self, sustain: f32) {
        let clamped_sustain = sustain.clamp(0.0, 1.0);
        self.sustain_var.set_value(clamped_sustain);
    }

    /// Get ADSR sustain level
    pub fn get_sustain(&self) -> f32 {
        self.sustain_var.value()
    }

    /// Set ADSR release time (in seconds)
    pub fn set_release(&mut self, release: f32) {
        let clamped_release = release.clamp(0.001, 10.0); // 1ms to 10s
        self.release_var.set_value(clamped_release);
    }

    /// Get ADSR release time
    pub fn get_release(&self) -> f32 {
        self.release_var.value()
    }

    /// Test function to verify dynamic waveform switching works
    #[cfg(test)]
    pub fn test_waveform_switching(&mut self) {
        println!("🧪 Testing dynamic waveform switching...");

        // Test switching between all waveforms
        let waveforms = [
            Waveform::Sine,
            Waveform::Square,
            Waveform::Sawtooth,
            Waveform::Triangle,
        ];

        for waveform in waveforms.iter() {
            self.set_waveform(*waveform);
            assert_eq!(self.get_waveform(), *waveform);
            println!("✅ Successfully switched to {}", waveform.as_str());

            // Generate a few samples to ensure it works
            for _ in 0..10 {
                let _sample = self.get_sample();
            }
        }

        println!("🎉 All waveform switches successful!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waveform_enum() {
        assert_eq!(Waveform::Sine.as_str(), "sine");
        assert_eq!(Waveform::Square.as_str(), "square");
        assert_eq!(Waveform::Sawtooth.as_str(), "sawtooth");
        assert_eq!(Waveform::Triangle.as_str(), "triangle");

        assert_eq!(Waveform::from_str("sine"), Some(Waveform::Sine));
        assert_eq!(Waveform::from_str("SQUARE"), Some(Waveform::Square));
        assert_eq!(Waveform::from_str("invalid"), None);
    }

    #[test]
    fn test_dynamic_waveform_switching() {
        let mut synth = FunDSPSynth::new(44100.0).expect("Failed to create synth");
        synth.test_waveform_switching();
    }
}
