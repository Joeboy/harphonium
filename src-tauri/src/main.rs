// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod commands;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // Initialize audio engine
            if let Err(e) = audio::initialize_audio() {
                eprintln!("Failed to initialize audio: {}", e);
                // Continue anyway - the app can still work without audio for UI development
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::play_note,
            commands::stop_note,
            commands::set_master_volume,
            commands::get_master_volume,
            commands::set_waveform,
            commands::get_waveform,
            commands::set_attack,
            commands::get_attack,
            commands::set_decay,
            commands::get_decay,
            commands::set_sustain,
            commands::get_sustain,
            commands::set_release,
            commands::get_release,
            commands::set_delay_time,
            commands::get_delay_time,
            commands::set_delay_mix,
            commands::get_delay_mix
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
