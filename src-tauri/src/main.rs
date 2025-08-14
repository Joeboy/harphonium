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
            get_master_volume
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
