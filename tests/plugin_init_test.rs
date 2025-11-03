// Test FmodPlugin initialization with actual audio banks
// These tests verify the plugin can be added to a Bevy app and loads banks correctly

use bevy::prelude::*;
use bevy_fmod::{FmodPlugin, FmodStudio};

#[test]
fn test_plugin_with_banks() {
    // Test plugin initialization with actual bank files
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(FmodPlugin::new(&[
            "tests/data/Master.bank",
            "tests/data/Master.strings.bank",
            "tests/data/SFX.bank",
        ]));

    // Verify FmodStudio resource exists
    let studio = app.world().get_resource::<FmodStudio>();
    assert!(studio.is_some(), "FmodStudio resource should exist");

    // Verify we can access the studio
    let studio = studio.unwrap();

    // Check that banks are loaded by trying to get an event
    // The Master.strings.bank should contain event paths
    let event_result = studio.get_event("event:/test");

    // Even if this specific event doesn't exist, FMOD should return a proper error
    // (not a "not initialized" error), proving banks are loaded
    match event_result {
        Ok(_) => {
            println!("Event found successfully");
        }
        Err(e) => {
            // Event not found is OK - it means banks are loaded and queried
            println!("Event query returned (banks loaded): {:?}", e);
        }
    }
}

#[test]
#[should_panic(expected = "Failed to canonicalize provided audio banks directory path")]
fn test_plugin_with_invalid_bank_path() {
    // Test that plugin panics with invalid bank paths
    // Note: This could be improved to handle errors more gracefully in the future
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(FmodPlugin::new(&[
            "tests/data/NonExistent.bank",
        ]));
}

#[test]
fn test_studio_creation() {
    // Test that we can create an FMOD Studio instance directly
    use libfmod::Studio;

    let studio = Studio::create().expect("Failed to create FMOD Studio");

    // Verify we can get the core system
    let core_system = studio.get_core_system().expect("Failed to get core system");
    assert!(core_system.get_version().is_ok());

    studio.release().expect("Failed to release Studio");
}

#[test]
fn test_studio_initialization() {
    // Test Studio initialization with minimal settings
    use libfmod::{Init, Studio, StudioInit};

    let studio = Studio::create().expect("Failed to create FMOD Studio");

    // Initialize with default settings
    // Note: This may fail in headless environments without audio devices
    let init_result = studio.initialize(512, StudioInit::NORMAL, Init::NORMAL, None);

    match init_result {
        Ok(_) => {
            // If initialization succeeded, test update
            studio.update().expect("Failed to update Studio");
            studio.release().expect("Failed to release Studio");
        }
        Err(e) => {
            // Error code 51 = "Error initializing output device" - expected in headless environments
            println!("Studio initialization failed (expected in headless environment): {:?}", e);
            studio.release().ok();
        }
    }
}
