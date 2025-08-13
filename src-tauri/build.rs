fn main() {
    tauri_build::build();

    // Add Android-specific build configuration to fix __cxa_pure_virtual issue
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("android") {
        // Link against the C++ standard library for Android
        println!("cargo:rustc-link-lib=dylib=c++_shared");

        // Additional C++ runtime symbols
        println!("cargo:rustc-link-lib=dylib=c++abi");
    }
}
