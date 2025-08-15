// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;

// Commands for desktop version
#[tauri::command]
async fn play_note(frequency: f32) {
    if let Err(e) = audio::play_frequency(frequency) {
        eprintln!("Error playing note: {}", e);
    }
}

#[tauri::command]
async fn stop_note() {
    if let Err(e) = audio::stop_audio() {
        eprintln!("Error stopping note: {}", e);
    }
}

#[tauri::command]
async fn set_master_volume(volume: f32) {
    if let Err(e) = audio::set_master_volume(volume) {
        eprintln!("Error setting master volume: {}", e);
    }
}

#[tauri::command]
async fn get_master_volume() -> f32 {
    audio::get_master_volume()
}

#[tauri::command]
async fn set_waveform(waveform: String) -> Result<(), String> {
    audio::set_waveform(&waveform)
}

#[tauri::command]
async fn get_waveform() -> String {
    audio::get_waveform()
}

#[tauri::command]
async fn set_attack(attack: f32) {
    if let Err(e) = audio::set_attack(attack) {
        eprintln!("Error setting attack: {}", e);
    }
}

#[tauri::command]
async fn get_attack() -> f32 {
    audio::get_attack()
}

#[tauri::command]
async fn set_decay(decay: f32) {
    if let Err(e) = audio::set_decay(decay) {
        eprintln!("Error setting decay: {}", e);
    }
}

#[tauri::command]
async fn get_decay() -> f32 {
    audio::get_decay()
}

#[tauri::command]
async fn set_sustain(sustain: f32) {
    if let Err(e) = audio::set_sustain(sustain) {
        eprintln!("Error setting sustain: {}", e);
    }
}

#[tauri::command]
async fn get_sustain() -> f32 {
    audio::get_sustain()
}

#[tauri::command]
async fn set_release(release: f32) {
    if let Err(e) = audio::set_release(release) {
        eprintln!("Error setting release: {}", e);
    }
}

#[tauri::command]
async fn get_release() -> f32 {
    audio::get_release()
}

fn main() {
    // Initialize audio engine
    if let Err(e) = audio::initialize_audio() {
        eprintln!("Failed to initialize audio: {}", e);
        // Continue anyway - the app can still work without audio for UI development
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            play_note, 
            stop_note,
            set_master_volume,
            get_master_volume,
            set_waveform,
            get_waveform,
            set_attack,
            get_attack,
            set_decay,
            get_decay,
            set_sustain,
            get_sustain,
            set_release,
            get_release
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
