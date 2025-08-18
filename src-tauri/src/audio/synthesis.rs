// Shared audio synthesis module using FunDSP
// This module provides FunDSP audio generation for both desktop and Android platforms

use fundsp::hacker::*;

use fundsp::buffer::{BufferArray, BufferRef};
use fundsp::hacker::{MAX_BUFFER_SIZE, U1};

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

    /// Fundsp node ids
    oscillator_nodeid: NodeId,
    adsr_nodeid: NodeId,
    delay_nodeid: NodeId,

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

    delay_time_var: shared::Shared,
    delay_feedback_var: shared::Shared,
    delay_mix_var: shared::Shared,

    /// Filter parameters
    filter_cutoff_var: shared::Shared,
    filter_resonance_var: shared::Shared,

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
        let attack_var = shared(0.02); // 50ms attack
        let decay_var = shared(0.2); // 200ms decay
        let sustain_var = shared(0.6); // 60% sustain level
        let release_var = shared(0.3); // 300ms release

        let delay_time_var = shared(0.3);
        let delay_feedback_var = shared(0.4);
        let delay_mix_var = shared(0.2);

        let filter_cutoff_var = shared(1000.0);
        let filter_resonance_var = shared(0.1);

        let mut net = Net::new(0, 1);

        // Create the synthesis chain dynamically
        let freq_dc_id = net.push(Box::new(var(&frequency_var)));

        let current_waveform = Waveform::default();
        let oscillator_nodeid = net.push(current_waveform.create_oscillator());
        net.pipe_all(freq_dc_id, oscillator_nodeid);

        // ADSR stuff
        let key_down_nodeid = net.push(Box::new(var(&key_down_var)));

        // Smoothing to try to mitigate audible clicks when retriggering the adsr
        let gate_smoother_id = net.push(Box::new(afollow(0.001, 0.001)));
        net.connect(key_down_nodeid, 0, gate_smoother_id, 0);

        let adsr_envelope = adsr_live(
            attack_var.value(),
            decay_var.value(),
            sustain_var.value(),
            release_var.value(),
        );
        let adsr_nodeid = net.push(Box::new(adsr_envelope));
        net.pipe_all(gate_smoother_id, adsr_nodeid);

        // More ADSR smoothing:
        // Keep this even shorter than the gate smoother so you don't blur transients.
        let env_micro_id = net.push(Box::new(afollow(0.0005, 0.0005)));
        net.connect(adsr_nodeid, 0, env_micro_id, 0);
        let vca_nodeid = net.push(Box::new(pass() * pass()));
        net.connect(oscillator_nodeid, 0, vca_nodeid, 0);
        net.connect(env_micro_id, 0, vca_nodeid, 1);

        // Delay stuff

        // Create mixer to feed delayed signal back to the delay node, mixed with the dry input signal
        let delay_feedback_gain_nodeid = net.push(Box::new(pass() * var(&delay_feedback_var)));
        let delay_feedback_mixer_nodeid = net.push(Box::new(pass() + pass()));
        net.connect(
            delay_feedback_gain_nodeid,
            0,
            delay_feedback_mixer_nodeid,
            1,
        );

        // Create delay node
        let delay_nodeid = net.push(Box::new(delay(delay_time_var.value())));
        // Connect the delay feedback mixer to the delay node
        net.connect(delay_feedback_mixer_nodeid, 0, delay_nodeid, 0);
        // Create delay gain node
        let delay_gain_nodeid = net.push(Box::new(pass() * var(&delay_mix_var)));
        // Create output mixer node
        // Mixes direct input, delay output
        let delay_output_mixer_nodeid = net.push(Box::new(pass() + pass()));
        // Wire direct input into output mixer node:
        net.connect(vca_nodeid, 0, delay_output_mixer_nodeid, 0);
        // Wire input into delay feedback mixer
        net.connect(vca_nodeid, 0, delay_feedback_mixer_nodeid, 0);
        // Wire delay output into delay mix node
        net.connect(delay_nodeid, 0, delay_gain_nodeid, 0);
        // Wire "gained" delay output into delay outputmixer node
        net.connect(delay_gain_nodeid, 0, delay_output_mixer_nodeid, 1);

        // Wire delay output into delay feedback mixer
        net.connect(delay_nodeid, 0, delay_feedback_gain_nodeid, 0);
        // net.connect(delay_feedback_mixer_nodeid, 0, delay_mixer_nodeid, 2);

        // Filter
        let filter_nodeid = net.push(Box::new(lowpass()));
        net.connect(delay_output_mixer_nodeid, 0, filter_nodeid, 0);
        let filter_cutoff_nodeid = net.push(Box::new(var(&filter_cutoff_var)));
        net.connect(filter_cutoff_nodeid, 0, filter_nodeid, 1);
        let filter_resonance_nodeid = net.push(Box::new(var(&filter_resonance_var)));
        net.connect(filter_resonance_nodeid, 0, filter_nodeid, 2);

        let master_vol_nodeid = net.push(Box::new(split() >> (pass() * var(&master_volume_var))));
        net.pipe_all(filter_nodeid, master_vol_nodeid);

        net.pipe_output(master_vol_nodeid);

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
            oscillator_nodeid,
            adsr_nodeid,
            delay_nodeid,

            current_waveform,
            frequency_var,
            key_down_var,
            master_volume_var,

            attack_var,
            decay_var,
            sustain_var,
            release_var,

            delay_time_var,
            delay_feedback_var,
            delay_mix_var,

            filter_cutoff_var,
            filter_resonance_var,

            sample_rate,
            enabled: true,
        })
    }

    /// Fill the output buffer with audio samples
    pub fn fill_buffer(&mut self, output: &mut [f32]) {
        if !self.enabled {
            output.fill(0.0);
            return;
        }

        let mut i = 0;
        while i < output.len() {
            // Work in chunks up to MAX_BUFFER_SIZE (usually 64 samples)
            let n = std::cmp::min(output.len() - i, MAX_BUFFER_SIZE);
            // Prepare an empty input buffer (no input channels)
            let input = BufferRef::empty();

            // Prepare an output buffer with 1 channel
            let mut block = BufferArray::<U1>::new();

            // Process a block of samples
            self.backend.process(n, &input, &mut block.buffer_mut());

            // Copy from the block into your output buffer, clamping each sample
            let ch = block.buffer_ref().channel_f32(0);
            for (dst, &src) in output[i..i + n].iter_mut().zip(&ch[..n]) {
                *dst = src.clamp(-1.0, 1.0);
            }

            i += n;
        }
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
            .replace(self.oscillator_nodeid, new_waveform.create_oscillator());

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

    /// Play a note at the specified frequency
    pub fn play_note(&mut self, frequency: f32) {
        if self.enabled {
            self.frequency_var.set_value(frequency);
            self.key_down_var.set_value(1.0); // Gate on - triggers ADSR attack
        }

        println!("Playing frequency: {} Hz", frequency);
    }

    /// Set note frequency (for violin / fretless mode)
    pub fn set_frequency(&mut self, frequency: f32) {
        if self.enabled {
            self.frequency_var.set_value(frequency);
        }
    }

    /// Stop the current note
    pub fn note_off(&mut self) {
        if self.enabled {
            self.key_down_var.set_value(0.0); // Gate off - triggers ADSR release
        }
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

    pub fn set_adsr(&mut self) {
        if !self.enabled {
            return; // No change needed
        }

        let attack = self.attack_var.value();
        let decay = self.decay_var.value();
        let sustain = self.sustain_var.value();
        let release = self.release_var.value();

        let new_adsr = Box::new(adsr_live(attack, decay, sustain, release));
        self.net.replace(self.adsr_nodeid, new_adsr);

        self.net.commit();
    }

    pub fn set_attack(&mut self, attack: f32) {
        println!("Setting attack to {}", attack);
        let clamped_attack = attack.clamp(0.001, 5.0); // 1ms to 5s
        self.attack_var.set_value(clamped_attack);
        self.set_adsr();
    }

    /// Get ADSR attack time
    pub fn get_attack(&self) -> f32 {
        self.attack_var.value()
    }

    /// Set ADSR decay time (in seconds)
    pub fn set_decay(&mut self, decay: f32) {
        let clamped_decay = decay.clamp(0.001, 5.0); // 1ms to 5s
        self.decay_var.set_value(clamped_decay);
        self.set_adsr();
    }

    /// Get ADSR decay time
    pub fn get_decay(&self) -> f32 {
        self.decay_var.value()
    }

    /// Set ADSR sustain level (0.0 to 1.0)
    pub fn set_sustain(&mut self, sustain: f32) {
        let clamped_sustain = sustain.clamp(0.0, 1.0);
        self.sustain_var.set_value(clamped_sustain);
        self.set_adsr();
    }

    /// Get ADSR sustain level
    pub fn get_sustain(&self) -> f32 {
        self.sustain_var.value()
    }

    /// Set ADSR release time (in seconds)
    pub fn set_release(&mut self, release: f32) {
        let clamped_release = release.clamp(0.001, 10.0); // 1ms to 10s
        self.release_var.set_value(clamped_release);
        self.set_adsr();
    }

    /// Get ADSR release time
    pub fn get_release(&self) -> f32 {
        self.release_var.value()
    }

    /// Set delay time (in seconds)
    pub fn set_delay_time(&mut self, delay_time: f32) {
        if !self.enabled {
            return; // No change needed
        }
        self.delay_time_var.set_value(delay_time.clamp(0.0, 5.0)); // Clamp to 0-5 seconds

        let new_delay = Box::new(delay(self.delay_time_var.value()));
        self.net.replace(self.delay_nodeid, new_delay);
        self.net.commit();
    }

    /// Get delay time (in seconds)
    pub fn get_delay_time(&self) -> f32 {
        self.delay_time_var.value()
    }

    /// Set delay feedback (0.0 to 1.0)
    pub fn set_delay_feedback(&mut self, feedback: f32) {
        if !self.enabled {
            return; // No change needed
        }
        self.delay_feedback_var.set_value(feedback.clamp(0.0, 1.0));
    }

    /// Get delay feedback
    pub fn get_delay_feedback(&self) -> f32 {
        self.delay_feedback_var.value()
    }

    pub fn set_delay_mix(&mut self, delay_mix: f32) {
        let clamped_delay_mix = delay_mix.clamp(0.0, 1.0); // 0% to 100%
        self.delay_mix_var.set_value(clamped_delay_mix);
    }

    /// Get delay mix (0.0 to 1.0)
    pub fn get_delay_mix(&self) -> f32 {
        self.delay_mix_var.value()
    }

    pub fn set_filter_cutoff(&mut self, cutoff: f32) {
        if !self.enabled {
            return; // No change needed
        }
        self.filter_cutoff_var
            .set_value(cutoff.clamp(20.0, 20000.0)); // 20 Hz to 20 kHz
    }

    /// Get filter cutoff frequency
    pub fn get_filter_cutoff(&self) -> f32 {
        self.filter_cutoff_var.value()
    }

    /// Set filter resonance (0.0 to 1.0)
    pub fn set_filter_resonance(&mut self, resonance: f32) {
        if !self.enabled {
            return; // No change needed
        }
        self.filter_resonance_var
            .set_value(resonance.clamp(0.0, 1.0));
    }

    /// Get filter resonance
    pub fn get_filter_resonance(&self) -> f32 {
        self.filter_resonance_var.value()
    }
}
