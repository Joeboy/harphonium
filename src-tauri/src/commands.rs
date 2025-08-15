// src-tauri/src/commands.rs
// All Tauri command functions live here and are imported by both lib.rs and main.rs

use crate::audio;

#[tauri::command]
pub async fn play_note(frequency: f32) {
    if let Err(e) = audio::play_frequency(frequency) {
        eprintln!("Error playing note: {}", e);
    }
}

#[tauri::command]
pub async fn stop_note() {
    if let Err(e) = audio::stop_audio() {
        eprintln!("Error stopping note: {}", e);
    }
}

#[tauri::command]
pub async fn set_master_volume(volume: f32) {
    if let Err(e) = audio::set_master_volume(volume) {
        eprintln!("Error setting master volume: {}", e);
    }
}

#[tauri::command]
pub async fn get_master_volume() -> f32 {
    audio::get_master_volume()
}

#[tauri::command]
pub async fn set_waveform(waveform: String) -> Result<(), String> {
    audio::set_waveform(&waveform)
}

#[tauri::command]
pub async fn get_waveform() -> String {
    audio::get_waveform()
}

#[tauri::command]
pub async fn set_attack(attack: f32) {
    if let Err(e) = audio::set_attack(attack) {
        eprintln!("Error setting attack: {}", e);
    }
}

#[tauri::command]
pub async fn get_attack() -> f32 {
    audio::get_attack()
}

#[tauri::command]
pub async fn set_decay(decay: f32) {
    if let Err(e) = audio::set_decay(decay) {
        eprintln!("Error setting decay: {}", e);
    }
}

#[tauri::command]
pub async fn get_decay() -> f32 {
    audio::get_decay()
}

#[tauri::command]
pub async fn set_sustain(sustain: f32) {
    if let Err(e) = audio::set_sustain(sustain) {
        eprintln!("Error setting sustain: {}", e);
    }
}

#[tauri::command]
pub async fn get_sustain() -> f32 {
    audio::get_sustain()
}

#[tauri::command]
pub async fn set_release(release: f32) {
    if let Err(e) = audio::set_release(release) {
        eprintln!("Error setting release: {}", e);
    }
}

#[tauri::command]
pub async fn get_release() -> f32 {
    audio::get_release()
}
