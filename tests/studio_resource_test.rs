// Test FmodStudio resource and component creation
use bevy::prelude::*;
use bevy_fmod::components::*;
use libfmod::{Init, Studio, StudioInit};
use libfmod::StopMode;

/// Helper to create and initialize FMOD Studio for testing
/// Returns None if initialization fails (e.g., in headless environments)
fn create_test_studio() -> Option<Studio> {
    let studio = Studio::create().expect("Failed to create FMOD Studio");

    // Initialize with default settings
    // This may fail in headless environments without audio devices
    match studio.initialize(512, StudioInit::NORMAL, Init::NORMAL, None) {
        Ok(_) => {
            let (version, build) = studio.get_core_system()
                .and_then(|sys| sys.get_version())
                .expect("Failed to get FMOD version");
            let major = (version >> 16) & 0xFFFF;
            let minor = (version >> 8) & 0xFF;
            let patch = version & 0xFF;
            println!("Test using FMOD Studio {}.{:02}.{:02} (build {})", major, minor, patch, build);
            Some(studio)
        }
        Err(e) => {
            println!("Skipping test - Studio initialization failed (headless environment): {:?}", e);
            studio.release().ok();
            None
        }
    }
}

#[test]
fn test_audio_source_component() {
    let Some(studio) = create_test_studio() else {
        return; // Skip test in headless environment
    };

    // Create a simple event description (this will fail without banks, which is expected)
    let event_result = studio.get_event("event:/NonExistent");

    // We expect this to fail since we don't have banks loaded
    assert!(
        event_result.is_err(),
        "Should fail to get event without banks loaded"
    );

    studio.release().ok();
}

#[test]
fn test_audio_listener_component() {
    // Test AudioListener component creation
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Spawn an entity with AudioListener
    app.world_mut().spawn(AudioListener);

    // Verify the component exists
    let mut query = app.world_mut().query::<&AudioListener>();
    assert_eq!(query.iter(app.world()).count(), 1);
}

#[test]
fn test_velocity_component() {
    // Test Velocity component
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    let velocity = Velocity::default();

    // Spawn an entity with Velocity
    app.world_mut().spawn(velocity);

    // Verify the component exists
    let mut query = app.world_mut().query::<&Velocity>();
    assert_eq!(query.iter(app.world()).count(), 1);
}

#[test]
fn test_stop_mode_values() {
    // Test that StopMode enum values are accessible
    let _immediate = StopMode::Immediate;
    let _allow_fadeout = StopMode::AllowFadeout;

    // These should compile and be usable
    assert!(true);
}

#[test]
fn test_spatial_bundle_creation() {
    let Some(studio) = create_test_studio() else {
        return; // Skip test in headless environment
    };

    // We can't create a full bundle without an event instance, but we can test the components
    let transform = Transform::default();
    let velocity = Velocity::default();
    let audio_listener = AudioListener;

    // Verify types compile
    let _t: Transform = transform;
    let _v: Velocity = velocity;
    let _a: AudioListener = audio_listener;

    studio.release().ok();
}
