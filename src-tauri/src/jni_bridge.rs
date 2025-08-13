//! JNI bindings for direct native audio control
//! This bypasses Tauri IPC for ultra-low latency audio triggers

use jni::objects::JClass;
use jni::sys::jfloat;
use jni::JNIEnv;
use std::sync::atomic::Ordering;

use crate::audio;

// JNI bindings for NativeAudioTouchHandler (touch overlay)
#[no_mangle]
pub extern "C" fn Java_uk_co_joebutton_synthmob_NativeAudioTouchHandler_nativePlayNote(
    _env: JNIEnv,
    _class: JClass,
    frequency: jfloat,
) {
    // CRITICAL PATH: Direct audio trigger - no IPC overhead
    let audio_state = audio::get_audio_state();

    // Ensure audio is initialized
    let _ = audio::initialize_audio();

    // Set frequency and start playing immediately
    audio_state
        .frequency_bits
        .store((frequency as f32).to_bits(), Ordering::Relaxed);
    audio_state.is_playing.store(true, Ordering::Relaxed);

    println!("Native note trigger (touch): {} Hz", frequency);
}

#[no_mangle]
pub extern "C" fn Java_uk_co_joebutton_synthmob_NativeAudioTouchHandler_nativeStopNote(
    _env: JNIEnv,
    _class: JClass,
) {
    // CRITICAL PATH: Direct audio stop - no IPC overhead
    let audio_state = audio::get_audio_state();
    audio_state.is_playing.store(false, Ordering::Relaxed);
    println!("Native note stop (touch)");
}

// JNI bindings for FastAudioInterface (WebView interface)
#[no_mangle]
pub extern "C" fn Java_uk_co_joebutton_synthmob_FastAudioInterface_nativePlayNote(
    _env: JNIEnv,
    _class: JClass,
    frequency: jfloat,
) {
    // CRITICAL PATH: Direct audio trigger via WebView interface - no IPC overhead
    let audio_state = audio::get_audio_state();

    // Ensure audio is initialized
    let _ = audio::initialize_audio();

    // Set frequency and start playing immediately
    audio_state
        .frequency_bits
        .store((frequency as f32).to_bits(), Ordering::Relaxed);
    audio_state.is_playing.store(true, Ordering::Relaxed);

    println!("Native note trigger (WebView): {} Hz", frequency);
}

#[no_mangle]
pub extern "C" fn Java_uk_co_joebutton_synthmob_FastAudioInterface_nativeStopNote(
    _env: JNIEnv,
    _class: JClass,
) {
    // CRITICAL PATH: Direct audio stop via WebView interface - no IPC overhead
    let audio_state = audio::get_audio_state();
    audio_state.is_playing.store(false, Ordering::Relaxed);
    println!("Native note stop (WebView)");
}
