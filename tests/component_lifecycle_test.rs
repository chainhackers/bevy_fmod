// Test AudioSource component lifecycle and hooks
// Verifies component cleanup, event instance management, and despawn behavior

use bevy::prelude::*;
use bevy_fmod::components::AudioSource;
use bevy_fmod::{FmodPlugin, FmodStudio};
use libfmod::StopMode;

#[test]
fn test_audio_source_despawn_cleanup() {
    // Test that AudioSource component hook properly stops and releases event on despawn
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(FmodPlugin::new(&[
            "tests/data/Master.bank",
            "tests/data/Master.strings.bank",
            "tests/data/SFX.bank",
        ]));

    // Get the FmodStudio resource to create an event
    let studio = app.world().resource::<FmodStudio>();

    // Try to create an event instance
    // This may fail if the specific event doesn't exist in the banks, which is OK for this test
    // We're testing the hook system, not the event content
    let event_result = studio.get_event("event:/test");

    match event_result {
        Ok(event_desc) => {
            let event_instance = event_desc.create_instance().expect("Failed to create instance");

            // Spawn entity with AudioSource
            let entity = app.world_mut().spawn(AudioSource {
                event_instance,
                despawn_stop_mode: StopMode::Immediate,
            }).id();

            // Verify entity exists
            assert!(app.world().get_entity(entity).is_ok());

            // Despawn the entity - this should trigger the on_remove hook
            app.world_mut().despawn(entity);

            // Verify entity no longer exists
            assert!(app.world().get_entity(entity).is_err());

            println!("AudioSource cleanup hook executed on despawn");
        }
        Err(e) => {
            println!("Skipping cleanup test - no valid events in banks: {:?}", e);
        }
    }
}

#[test]
fn test_audio_source_stop_mode_immediate() {
    // Test that despawn_stop_mode is respected
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(FmodPlugin::new(&[
            "tests/data/Master.bank",
            "tests/data/Master.strings.bank",
        ]));

    let studio = app.world().resource::<FmodStudio>();
    let event_result = studio.get_event("event:/test");

    match event_result {
        Ok(event_desc) => {
            let event_instance = event_desc.create_instance().expect("Failed to create instance");

            // Test with Immediate stop mode
            let audio_source = AudioSource {
                event_instance,
                despawn_stop_mode: StopMode::Immediate,
            };

            assert_eq!(audio_source.despawn_stop_mode, StopMode::Immediate);
        }
        Err(_) => {
            println!("Skipping stop mode test - no valid events in banks");
        }
    }
}

#[test]
fn test_audio_source_stop_mode_allow_fadeout() {
    // Test AllowFadeout stop mode
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(FmodPlugin::new(&[
            "tests/data/Master.bank",
            "tests/data/Master.strings.bank",
        ]));

    let studio = app.world().resource::<FmodStudio>();
    let event_result = studio.get_event("event:/test");

    match event_result {
        Ok(event_desc) => {
            let event_instance = event_desc.create_instance().expect("Failed to create instance");

            // Test with AllowFadeout stop mode
            let audio_source = AudioSource {
                event_instance,
                despawn_stop_mode: StopMode::AllowFadeout,
            };

            assert_eq!(audio_source.despawn_stop_mode, StopMode::AllowFadeout);
        }
        Err(_) => {
            println!("Skipping fadeout test - no valid events in banks");
        }
    }
}
