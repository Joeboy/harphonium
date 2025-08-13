// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

// Simple synth state for now - we'll expand this later with Oboe integration
struct SynthState {
    is_playing: bool,
    current_frequency: f32,
}

// Create a global state for the synth
static SYNTH_STATE: Mutex<SynthState> = Mutex::new(SynthState {
    is_playing: false,
    current_frequency: 440.0,
});

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn play_note(frequency: f32) -> String {
    let mut state = SYNTH_STATE.lock().unwrap();
    state.is_playing = true;
    state.current_frequency = frequency;
    
    // TODO: This is where we'll integrate with Oboe for actual audio synthesis
    format!("Playing note at {} Hz", frequency)
}

#[tauri::command]
fn stop_note() -> String {
    let mut state = SYNTH_STATE.lock().unwrap();
    state.is_playing = false;
    
    // TODO: Stop the audio synthesis
    "Note stopped".to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![play_note, stop_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
