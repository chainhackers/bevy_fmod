//! This build script configures FMOD library linking.
//!
//! ## Linux Setup
//! Set the FMOD_SDK_DIR environment variable to point to your FMOD SDK directory.
//! Example: export FMOD_SDK_DIR=/path/to/fmodstudioapi20310linux
//!
//! You also need to set LD_LIBRARY_PATH to include the libraries at runtime:
//! export LD_LIBRARY_PATH=$FMOD_SDK_DIR/api/core/lib/x86_64:$FMOD_SDK_DIR/api/studio/lib/x86_64:$LD_LIBRARY_PATH
//!
//! ## Alternative Methods
//! 1. Use the provided ./run_tests.sh script for testing
//! 2. Configure .cargo/config.toml with library paths (see .cargo/config.toml.example)
//! 3. Copy FMOD libraries to system locations (/usr/local/lib, /usr/lib)

fn main() {
    #[cfg(target_os = "linux")]
    {
        let mut target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

        // Account for the naming mismatch between FMOD library folders and the target architecture
        if target_arch == "aarch64" {
            target_arch = "arm64".into();
        }

        // Try to find FMOD libraries using environment variable
        if let Ok(fmod_dir) = std::env::var("FMOD_SDK_DIR") {
            println!("cargo:rerun-if-env-changed=FMOD_SDK_DIR");
            println!("cargo:warning=Using FMOD SDK from: {}", fmod_dir);

            let api_loc = [
                format!("{}/api/core/lib/{}", fmod_dir, target_arch),
                format!("{}/api/studio/lib/{}", fmod_dir, target_arch),
            ];

            let mut found_any = false;
            for loc in &api_loc {
                // Verify the directory exists
                if std::path::Path::new(loc).exists() {
                    println!("cargo:rustc-link-search={}", loc);
                    println!("cargo:rustc-env=LD_RUN_PATH={}", loc);
                    found_any = true;
                } else {
                    println!("cargo:warning=FMOD library directory not found: {}", loc);
                }
            }

            if !found_any {
                println!("cargo:warning=No FMOD libraries found in specified SDK directory!");
                println!("cargo:warning=Checked architecture: {}", target_arch);
            }
        } else {
            // Fall back to common system locations
            let system_paths = [
                "/usr/local/lib",
                "/usr/lib",
                "/opt/fmod/lib",
            ];

            for path in &system_paths {
                if std::path::Path::new(path).exists() {
                    println!("cargo:rustc-link-search={}", path);
                }
            }

            println!("cargo:warning=FMOD_SDK_DIR environment variable not set. Set it to your FMOD SDK directory.");
            println!("cargo:warning=Example: export FMOD_SDK_DIR=/path/to/fmod/20309_processed");
        }
    }
}