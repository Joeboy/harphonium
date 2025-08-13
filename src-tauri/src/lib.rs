// Mobile library interface for SynthMob
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;

// Commands for both mobile and desktop
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

// Mobile library entry point
#[cfg(mobile)]
#[tauri::mobile_entry_point]
pub fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // Initialize audio engine
            if let Err(e) = audio::initialize_audio() {
                eprintln!("Failed to initialize audio: {}", e);
                // Continue anyway - the app can still work without audio for UI development
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![play_note, stop_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
