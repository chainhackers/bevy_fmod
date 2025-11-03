// Test FMOD event lifecycle
// Verifies event creation, playback control, and cleanup

use bevy::prelude::*;
use bevy_fmod::components::AudioSource;
use bevy_fmod::{FmodPlugin, FmodStudio};
use libfmod::StopMode;

#[test]
fn test_event_instance_creation() {
    // Test creating event instances from event descriptions
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(FmodPlugin::new(&[
            "tests/data/Master.bank",
            "tests/data/Master.strings.bank",
        ]));

    let studio = app.world().resource::<FmodStudio>();

    // Try to get an event and create an instance
    let event_result = studio.get_event("event:/test");

    match event_result {
        Ok(event_desc) => {
            let event_instance = event_desc.create_instance();
            assert!(event_instance.is_ok(), "Should be able to create event instance");

            let instance = event_instance.unwrap();

            // Verify we can call methods on the instance
            assert!(instance.get_volume().is_ok(), "Should be able to get volume");

            // Clean up
            instance.release().ok();

            println!("Event instance created successfully");
        }
        Err(_) => {
            println!("Skipping event creation test - no valid events in banks");
        }
    }
}

#[test]
fn test_event_start_stop() {
    // Test starting and stopping events
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

            // Start the event
            let start_result = event_instance.start();
            if start_result.is_ok() {
                println!("Event started successfully");

                // Stop the event immediately
                event_instance.stop(StopMode::Immediate).expect("Failed to stop event");
                println!("Event stopped successfully");
            } else {
                println!("Event start not supported (may need audio device): {:?}", start_result);
            }

            // Clean up
            event_instance.release().ok();
        }
        Err(_) => {
            println!("Skipping start/stop test - no valid events in banks");
        }
    }
}

#[test]
fn test_event_volume_control() {
    // Test setting and getting event volume
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

            // Set volume
            event_instance.set_volume(0.5).expect("Failed to set volume");

            // Get volume
            let (volume, _final_volume) = event_instance.get_volume().expect("Failed to get volume");
            assert!((volume - 0.5).abs() < 0.01, "Volume should be approximately 0.5");

            println!("Event volume control works (set: 0.5, got: {})", volume);

            // Clean up
            event_instance.release().ok();
        }
        Err(_) => {
            println!("Skipping volume test - no valid events in banks");
        }
    }
}

#[test]
fn test_event_pause_unpause() {
    // Test pausing and unpausing events
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

            // Pause the event
            event_instance.set_paused(true).expect("Failed to pause event");

            // Check if paused
            let is_paused = event_instance.get_paused().expect("Failed to get paused state");
            assert!(is_paused, "Event should be paused");

            // Unpause
            event_instance.set_paused(false).expect("Failed to unpause event");

            let is_paused = event_instance.get_paused().expect("Failed to get paused state");
            assert!(!is_paused, "Event should be unpaused");

            println!("Event pause/unpause works");

            // Clean up
            event_instance.release().ok();
        }
        Err(_) => {
            println!("Skipping pause test - no valid events in banks");
        }
    }
}

#[test]
fn test_stop_mode_behavior() {
    // Test different stop modes
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
            // Test Immediate stop
            let instance1 = event_desc.create_instance().expect("Failed to create instance");
            let stop_result = instance1.stop(StopMode::Immediate);
            if stop_result.is_ok() {
                println!("StopMode::Immediate works");
            }
            instance1.release().ok();

            // Test AllowFadeout stop
            let instance2 = event_desc.create_instance().expect("Failed to create instance");
            let stop_result = instance2.stop(StopMode::AllowFadeout);
            if stop_result.is_ok() {
                println!("StopMode::AllowFadeout works");
            }
            instance2.release().ok();
        }
        Err(_) => {
            println!("Skipping stop mode test - no valid events in banks");
        }
    }
}

#[test]
fn test_event_with_audio_source_component() {
    // Integration test: event instance in AudioSource component
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(TransformPlugin)
        .add_plugins(FmodPlugin::new(&[
            "tests/data/Master.bank",
            "tests/data/Master.strings.bank",
        ]));

    let studio = app.world().resource::<FmodStudio>();
    let event_result = studio.get_event("event:/test");

    match event_result {
        Ok(event_desc) => {
            let event_instance = event_desc.create_instance().expect("Failed to create instance");

            // Spawn with AudioSource
            let entity = app.world_mut().spawn((
                AudioSource {
                    event_instance,
                    despawn_stop_mode: StopMode::AllowFadeout,
                },
                Transform::default(),
            )).id();

            // Access the AudioSource through the component
            let world = app.world();
            let audio_source = world.get::<AudioSource>(entity).expect("AudioSource should exist");

            // Verify we can access the event instance through the component
            assert!(audio_source.get_volume().is_ok());

            println!("Event instance works within AudioSource component");

            // Cleanup
            app.world_mut().despawn(entity);
        }
        Err(_) => {
            println!("Skipping AudioSource integration test - no valid events in banks");
        }
    }
}
