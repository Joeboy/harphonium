// src-tauri/src/commands.rs
// All Tauri command functions live here and are imported by both lib.rs and main.rs

use crate::audio::{handle_audio_event, AudioEvent, AudioEventResult, Waveform};

/// Play a note (piano mode)
#[tauri::command]
pub async fn play_note(frequency: f32) {
    match handle_audio_event(AudioEvent::PlayNote { frequency }) {
        AudioEventResult::Ok => (),
        AudioEventResult::Err(e) => {
            eprintln!("Error handling audio event: {}", e);
        }
        _ => {
            eprintln!("Unexpected result");
        }
    }
}

/// Set the frequency, for violin / fretless mode where
#[tauri::command]
pub async fn set_frequency(frequency: f32) {
    match handle_audio_event(AudioEvent::SetFrequency { frequency }) {
        AudioEventResult::Ok => (),
        AudioEventResult::Err(e) => {
            eprintln!("Error handling audio event: {}", e);
        }
        _ => {
            eprintln!("Unexpected result");
        }
    }
}

#[tauri::command]
pub async fn note_off() {
    match handle_audio_event(AudioEvent::NoteOff) {
        AudioEventResult::Ok => (),
        AudioEventResult::Err(e) => {
            eprintln!("Error handling audio event: {}", e);
        }
        _ => {
            eprintln!("Unexpected result");
        }
    }
}

#[tauri::command]
pub async fn set_master_volume(volume: f32) {
    match handle_audio_event(AudioEvent::SetMasterVolume { volume }) {
        AudioEventResult::Ok => (),
        AudioEventResult::Err(e) => {
            eprintln!("Error handling audio event: {}", e);
        }
        _ => {
            eprintln!("Unexpected result");
        }
    }
}

#[tauri::command]
pub async fn get_master_volume() -> f32 {
    // audio::get_master_volume()
    match handle_audio_event(AudioEvent::GetMasterVolume) {
        AudioEventResult::ValueF32(volume) => volume,
        AudioEventResult::Err(e) => {
            eprintln!("Error getting master volume: {}", e);
            0.0 // Return a default value on error
        }
        _ => {
            eprintln!("Unexpected result");
            0.0 // Return a default value on unexpected result
        }
    }
}

#[tauri::command]
pub async fn set_waveform(waveform: String) {
    match handle_audio_event(AudioEvent::SetWaveform {
        waveform: Waveform::from_str(&waveform).unwrap(),
    }) {
        AudioEventResult::Ok => (),
        AudioEventResult::Err(e) => {
            eprintln!("Error setting waveform: {}", e);
        }
        _ => {
            eprintln!("Unexpected result");
        }
    }
}

#[tauri::command]
pub async fn get_waveform() -> String {
    match handle_audio_event(AudioEvent::GetWaveform) {
        AudioEventResult::ValueWaveform(waveform) => waveform.as_str().to_string(),
        AudioEventResult::Err(e) => {
            eprintln!("Error getting waveform: {}", e);
            String::new() // Return a default value on error
        }
        _ => {
            eprintln!("Unexpected result");
            String::new() // Return a default value on unexpected result
        }
    }
}

#[tauri::command]
pub async fn set_attack(attack: f32) {
    match handle_audio_event(AudioEvent::SetAttack { attack }) {
        AudioEventResult::Ok => (),
        AudioEventResult::Err(e) => {
            eprintln!("Error setting attack: {}", e);
        }
        _ => {
            eprintln!("Unexpected result");
        }
    }
}

#[tauri::command]
pub async fn get_attack() -> f32 {
    match handle_audio_event(AudioEvent::GetAttack) {
        AudioEventResult::ValueF32(attack) => attack,
        AudioEventResult::Err(e) => {
            eprintln!("Error getting attack: {}", e);
            0.0 // Return a default value on error
        }
        _ => {
            eprintln!("Unexpected result");
            0.0 // Return a default value on unexpected result
        }
    }
}

#[tauri::command]
pub async fn set_decay(decay: f32) {
    match handle_audio_event(AudioEvent::SetDecay { decay }) {
        AudioEventResult::Ok => (),
        AudioEventResult::Err(e) => {
            eprintln!("Error setting decay: {}", e);
        }
        _ => {
            eprintln!("Unexpected result");
        }
    }
}

#[tauri::command]
pub async fn get_decay() -> f32 {
    match handle_audio_event(AudioEvent::GetDecay) {
        AudioEventResult::ValueF32(decay) => decay,
        AudioEventResult::Err(e) => {
            eprintln!("Error getting decay: {}", e);
            0.0 // Return a default value on error
        }
        _ => {
            eprintln!("Unexpected result");
            0.0 // Return a default value on unexpected result
        }
    }
}

#[tauri::command]
pub async fn set_sustain(sustain: f32) {
    match handle_audio_event(AudioEvent::SetSustain { sustain }) {
        AudioEventResult::Ok => (),
        AudioEventResult::Err(e) => {
            eprintln!("Error setting sustain: {}", e);
        }
        _ => {
            eprintln!("Unexpected result");
        }
    }
}

#[tauri::command]
pub async fn get_sustain() -> f32 {
    match handle_audio_event(AudioEvent::GetSustain) {
        AudioEventResult::ValueF32(sustain) => sustain,
        AudioEventResult::Err(e) => {
            eprintln!("Error getting sustain: {}", e);
            0.0 // Return a default value on error
        }
        _ => {
            eprintln!("Unexpected result");
            0.0 // Return a default value on unexpected result
        }
    }
}

#[tauri::command]
pub async fn set_release(release: f32) {
    match handle_audio_event(AudioEvent::SetRelease { release }) {
        AudioEventResult::Ok => (),
        AudioEventResult::Err(e) => {
            eprintln!("Error setting release: {}", e);
        }
        _ => {
            eprintln!("Unexpected result");
        }
    }
}

#[tauri::command]
pub async fn get_release() -> f32 {
    match handle_audio_event(AudioEvent::GetRelease) {
        AudioEventResult::ValueF32(release) => release,
        AudioEventResult::Err(e) => {
            eprintln!("Error getting release: {}", e);
            0.0 // Return a default value on error
        }
        _ => {
            eprintln!("Unexpected result");
            0.0 // Return a default value on unexpected result
        }
    }
}

#[tauri::command]
pub async fn set_delay_time(delay_time: f32) {
    match handle_audio_event(AudioEvent::SetDelayTime { delay_time }) {
        AudioEventResult::Ok => (),
        AudioEventResult::Err(e) => {
            eprintln!("Error setting delay time: {}", e);
        }
        _ => {
            eprintln!("Unexpected result");
        }
    }
}

#[tauri::command]
pub async fn get_delay_time() -> f32 {
    match handle_audio_event(AudioEvent::GetDelayTime) {
        AudioEventResult::ValueF32(delay_time) => delay_time,
        AudioEventResult::Err(e) => {
            eprintln!("Error getting delay time: {}", e);
            0.0 // Return a default value on error
        }
        _ => {
            eprintln!("Unexpected result");
            0.0 // Return a default value on unexpected result
        }
    }
}

#[tauri::command]
pub async fn set_delay_feedback(delay_feedback: f32) {
    match handle_audio_event(AudioEvent::SetDelayFeedback { delay_feedback }) {
        AudioEventResult::Ok => (),
        AudioEventResult::Err(e) => {
            eprintln!("Error setting delay feedback: {}", e);
        }
        _ => {
            eprintln!("Unexpected result");
        }
    }
}

#[tauri::command]
pub async fn get_delay_feedback() -> f32 {
    match handle_audio_event(AudioEvent::GetDelayFeedback) {
        AudioEventResult::ValueF32(delay_feedback) => delay_feedback,
        AudioEventResult::Err(e) => {
            eprintln!("Error getting delay feedback: {}", e);
            0.0 // Return a default value on error
        }
        _ => {
            eprintln!("Unexpected result");
            0.0 // Return a default value on unexpected result
        }
    }
}

#[tauri::command]
pub async fn set_delay_mix(delay_mix: f32) {
    match handle_audio_event(AudioEvent::SetDelayMix { delay_mix }) {
        AudioEventResult::Ok => (),
        AudioEventResult::Err(e) => {
            eprintln!("Error setting delay mix: {}", e);
        }
        _ => {
            eprintln!("Unexpected result");
        }
    }
}

#[tauri::command]
pub async fn get_delay_mix() -> f32 {
    match handle_audio_event(AudioEvent::GetDelayMix) {
        AudioEventResult::ValueF32(delay_mix) => delay_mix,
        AudioEventResult::Err(e) => {
            eprintln!("Error getting delay mix: {}", e);
            0.0 // Return a default value on error
        }
        _ => {
            eprintln!("Unexpected result");
            0.0 // Return a default value on unexpected result
        }
    }
}

#[tauri::command]
pub async fn set_filter_cutoff(cutoff: f32) {
    match handle_audio_event(AudioEvent::SetFilterCutoff { cutoff }) {
        AudioEventResult::Ok => (),
        AudioEventResult::Err(e) => {
            eprintln!("Error setting filter cutoff: {}", e);
        }
        _ => {
            eprintln!("Unexpected result");
        }
    }
}

#[tauri::command]
pub async fn get_filter_cutoff() -> f32 {
    match handle_audio_event(AudioEvent::GetFilterCutoff) {
        AudioEventResult::ValueF32(cutoff) => cutoff,
        AudioEventResult::Err(e) => {
            eprintln!("Error getting filter cutoff: {}", e);
            0.0 // Return a default value on error
        }
        _ => {
            eprintln!("Unexpected result");
            0.0 // Return a default value on unexpected result
        }
    }
}

#[tauri::command]
pub async fn set_filter_resonance(resonance: f32) {
    match handle_audio_event(AudioEvent::SetFilterResonance { resonance }) {
        AudioEventResult::Ok => (),
        AudioEventResult::Err(e) => {
            eprintln!("Error setting filter resonance: {}", e);
        }
        _ => {
            eprintln!("Unexpected result");
        }
    }
}

#[tauri::command]
pub async fn get_filter_resonance() -> f32 {
    match handle_audio_event(AudioEvent::GetFilterResonance) {
        AudioEventResult::ValueF32(resonance) => resonance,
        AudioEventResult::Err(e) => {
            eprintln!("Error getting filter resonance: {}", e);
            0.0 // Return a default value on error
        }
        _ => {
            eprintln!("Unexpected result");
            0.0 // Return a default value on unexpected result
        }
    }
}
