/// Audio synthesis module using FunDSP
use fundsp::buffer::{BufferArray, BufferRef};
use fundsp::hacker::{
    adsr_live, afollow, dcblock, delay, limiter, lowpass, pass, saw, shared, sine, split, square,
    triangle, var, AudioUnit, Net, NodeId, MAX_BUFFER_SIZE, U1,
};
use rtrb::Consumer;
use std::collections::HashMap;

pub fn drain_and_coalesce_events(consumer: &mut Consumer<AudioEvent>) -> Vec<AudioEvent> {
    let mut last_events: HashMap<&'static str, AudioEvent> = HashMap::new();
    let mut passthrough_events = Vec::new();

    while let Ok(event) = consumer.pop() {
        match &event {
            AudioEvent::SetFrequency { .. } => {
                last_events.insert("SetFrequency", event);
            }
            AudioEvent::SetMasterVolume { .. } => {
                last_events.insert("SetMasterVolume", event);
            }
            AudioEvent::SetWaveform { .. } => {
                last_events.insert("SetWaveform", event);
            }
            AudioEvent::SetAttack { .. } => {
                last_events.insert("SetAttack", event);
            }
            AudioEvent::SetDecay { .. } => {
                last_events.insert("SetDecay", event);
            }
            AudioEvent::SetSustain { .. } => {
                last_events.insert("SetSustain", event);
            }
            AudioEvent::SetRelease { .. } => {
                last_events.insert("SetRelease", event);
            }
            AudioEvent::SetDelayTime { .. } => {
                last_events.insert("SetDelayTime", event);
            }
            AudioEvent::SetDelayFeedback { .. } => {
                last_events.insert("SetDelayFeedback", event);
            }
            AudioEvent::SetDelayMix { .. } => {
                last_events.insert("SetDelayMix", event);
            }
            AudioEvent::SetFilterCutoff { .. } => {
                last_events.insert("SetFilterCutoff", event);
            }
            AudioEvent::SetFilterResonance { .. } => {
                last_events.insert("SetFilterResonance", event);
            }
            // Non-coalescable events (e.g., PlayNote, NoteOff, queries) go straight through
            _ => passthrough_events.push(event),
        }
    }
    passthrough_events.extend(last_events.into_values());
    passthrough_events
}

/// Enum representing all possible audio commands/events
#[derive(Debug)]
pub enum AudioEvent {
    PlayNote { frequency: f32 },
    SetFrequency { frequency: f32 },
    NoteOff,
    SetMasterVolume { volume: f32 },
    SetWaveform { waveform: Waveform },
    SetAttack { attack: f32 },
    SetDecay { decay: f32 },
    SetSustain { sustain: f32 },
    SetRelease { release: f32 },
    SetDelayTime { delay_time: f32 },
    SetDelayFeedback { delay_feedback: f32 },
    SetDelayMix { delay_mix: f32 },
    SetFilterCutoff { cutoff: f32 },
    SetFilterResonance { resonance: f32 },
    // Query events:
    GetMasterVolume,
    GetWaveform,
    GetAttack,
    GetDecay,
    GetSustain,
    GetRelease,
    GetDelayTime,
    GetDelayFeedback,
    GetDelayMix,
    GetFilterCutoff,
    GetFilterResonance,
}

#[derive(Debug)]
pub enum AudioEventResult {
    Ok,
    ValueF32(f32),
    // ValueString(String),
    ValueWaveform(Waveform),
    Err(String),
}

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
    // pub queue: AudioEventQueue,
    event_consumer: rtrb::Consumer<AudioEvent>,
}

impl FunDSPSynth {
    #[allow(dead_code)]
    pub fn new(
        sample_rate: f32,
        event_consumer: rtrb::Consumer<AudioEvent>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // let queue = AudioEventQueue::new(64);

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
        let freq_smooth_id = net.push(Box::new(afollow(0.001, 0.001)));
        net.connect(freq_dc_id, 0, freq_smooth_id, 0);

        let current_waveform = Waveform::default();
        let oscillator_nodeid = net.push(current_waveform.create_oscillator());
        net.pipe_all(freq_smooth_id, oscillator_nodeid);

        // Try to avoid clipping
        let pad_volume_nodeid = net.push(Box::new(pass() * 0.5));
        net.connect(oscillator_nodeid, 0, pad_volume_nodeid, 0);

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
        let env_micro_id = net.push(Box::new(afollow(0.0005, 0.0005)));
        net.connect(adsr_nodeid, 0, env_micro_id, 0);
        let vca_nodeid = net.push(Box::new(pass() * pass()));
        net.connect(pad_volume_nodeid, 0, vca_nodeid, 0);
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

        let dcblock_id = net.push(Box::new(dcblock()));
        net.pipe_all(master_vol_nodeid, dcblock_id);

        let limiter_id = net.push(Box::new(limiter(0.003, 0.050)));
        net.pipe_all(dcblock_id, limiter_id);

        net.pipe_output(limiter_id);

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
            event_consumer,
        })
    }

    #[allow(dead_code)]
    pub fn fill_buffer(&mut self, output: &mut [f32]) {
        if !self.enabled {
            output.fill(0.0);
            return;
        }
        let events = drain_and_coalesce_events(&mut self.event_consumer);
        for event in events {
            self.handle_event(event);
        }

        let mut i = 0;
        let mut block = BufferArray::<U1>::new();
        let input = BufferRef::empty();
        while i < output.len() {
            // Work in chunks up to MAX_BUFFER_SIZE (usually 64 samples)
            let n = std::cmp::min(output.len() - i, MAX_BUFFER_SIZE);
            self.backend.process(n, &input, &mut block.buffer_mut());

            // Copy from the block into the output buffer, clamping each sample
            let ch = block.buffer_ref().channel_f32(0);
            for (dst, &src) in output[i..i + n].iter_mut().zip(&ch[..n]) {
                *dst = src.clamp(-1.0, 1.0);
            }

            i += n;
        }
    }

    /// Update the backend sample rate and reset safely.
    #[allow(dead_code)]
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

        // println!("Playing frequency: {} Hz", frequency);
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

    /// Route UI events to the appropriate methods
    pub fn handle_event(&mut self, event: AudioEvent) -> AudioEventResult {
        match event {
            AudioEvent::PlayNote { frequency } => {
                self.play_note(frequency);
                AudioEventResult::Ok
            }
            AudioEvent::SetFrequency { frequency } => {
                self.set_frequency(frequency);
                AudioEventResult::Ok
            }
            AudioEvent::NoteOff => {
                self.note_off();
                AudioEventResult::Ok
            }
            AudioEvent::SetMasterVolume { volume } => {
                self.set_master_volume(volume);
                AudioEventResult::Ok
            }
            AudioEvent::SetWaveform { waveform } => {
                self.set_waveform(waveform);
                AudioEventResult::Ok
            }
            AudioEvent::SetAttack { attack } => {
                self.set_attack(attack);
                AudioEventResult::Ok
            }
            AudioEvent::SetDecay { decay } => {
                self.set_decay(decay);
                AudioEventResult::Ok
            }
            AudioEvent::SetSustain { sustain } => {
                self.set_sustain(sustain);
                AudioEventResult::Ok
            }
            AudioEvent::SetRelease { release } => {
                self.set_release(release);
                AudioEventResult::Ok
            }
            AudioEvent::SetDelayTime { delay_time } => {
                self.set_delay_time(delay_time);
                AudioEventResult::Ok
            }
            AudioEvent::SetDelayFeedback { delay_feedback } => {
                self.set_delay_feedback(delay_feedback);
                AudioEventResult::Ok
            }
            AudioEvent::SetDelayMix { delay_mix } => {
                self.set_delay_mix(delay_mix);
                AudioEventResult::Ok
            }
            AudioEvent::SetFilterCutoff { cutoff } => {
                self.set_filter_cutoff(cutoff);
                AudioEventResult::Ok
            }
            AudioEvent::SetFilterResonance { resonance } => {
                self.set_filter_resonance(resonance);
                AudioEventResult::Ok
            }
            AudioEvent::GetMasterVolume => AudioEventResult::ValueF32(self.get_master_volume()),
            AudioEvent::GetWaveform => AudioEventResult::ValueWaveform(self.get_waveform()),
            AudioEvent::GetAttack => AudioEventResult::ValueF32(self.get_attack()),
            AudioEvent::GetDecay => AudioEventResult::ValueF32(self.get_decay()),
            AudioEvent::GetSustain => AudioEventResult::ValueF32(self.get_sustain()),
            AudioEvent::GetRelease => AudioEventResult::ValueF32(self.get_release()),
            AudioEvent::GetDelayTime => AudioEventResult::ValueF32(self.get_delay_time()),
            AudioEvent::GetDelayFeedback => AudioEventResult::ValueF32(self.get_delay_feedback()),
            AudioEvent::GetDelayMix => AudioEventResult::ValueF32(self.get_delay_mix()),
            AudioEvent::GetFilterCutoff => AudioEventResult::ValueF32(self.get_filter_cutoff()),
            AudioEvent::GetFilterResonance => {
                AudioEventResult::ValueF32(self.get_filter_resonance())
            }
        }
    }
}
