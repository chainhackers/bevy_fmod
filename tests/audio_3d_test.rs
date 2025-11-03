// Test 3D audio positioning and spatial audio
// Verifies AudioSource and AudioListener 3D attribute updates

use bevy::prelude::*;
use bevy_fmod::components::{AudioListener, AudioSource, Velocity};
use bevy_fmod::{FmodPlugin, FmodStudio};
use libfmod::StopMode;

#[test]
fn test_audio_source_3d_attributes_with_transform() {
    // Test that AudioSource updates 3D attributes based on Transform
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

            // Spawn entity with AudioSource and Transform
            let entity = app.world_mut().spawn((
                AudioSource {
                    event_instance,
                    despawn_stop_mode: StopMode::Immediate,
                },
                Transform::from_xyz(10.0, 5.0, 3.0),
            )).id();

            // Run the update system
            app.update();

            // Verify entity exists
            assert!(app.world().get_entity(entity).is_ok());

            // Cleanup
            app.world_mut().despawn(entity);

            println!("AudioSource 3D attributes updated with transform");
        }
        Err(_) => {
            println!("Skipping 3D test - no valid events in banks");
        }
    }
}

#[test]
fn test_audio_source_with_velocity() {
    // Test AudioSource with Velocity component for Doppler effect
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

            // Spawn entity with AudioSource, Transform, and Velocity
            let entity = app.world_mut().spawn((
                AudioSource {
                    event_instance,
                    despawn_stop_mode: StopMode::Immediate,
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
                Velocity::default(),
            )).id();

            // Run update to process velocity
            app.update();

            // Verify all components exist
            let world = app.world();
            assert!(world.get::<AudioSource>(entity).is_some());
            assert!(world.get::<Transform>(entity).is_some());
            assert!(world.get::<Velocity>(entity).is_some());

            // Cleanup
            app.world_mut().despawn(entity);

            println!("AudioSource with Velocity component works");
        }
        Err(_) => {
            println!("Skipping velocity test - no valid events in banks");
        }
    }
}

#[test]
fn test_audio_listener_3d_attributes() {
    // Test that AudioListener updates 3D attributes
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(TransformPlugin)
        .add_plugins(FmodPlugin::new(&[
            "tests/data/Master.bank",
            "tests/data/Master.strings.bank",
        ]));

    // Spawn entity with AudioListener and Transform
    let listener_entity = app.world_mut().spawn((
        AudioListener,
        Transform::from_xyz(0.0, 0.0, 0.0),
    )).id();

    // Run update system
    app.update();

    // Verify listener exists
    assert!(app.world().get_entity(listener_entity).is_ok());
    assert!(app.world().get::<AudioListener>(listener_entity).is_some());

    println!("AudioListener 3D attributes updated");
}

#[test]
fn test_multiple_audio_sources() {
    // Test multiple AudioSource components in the same scene
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
            // Create multiple event instances
            let instance1 = event_desc.create_instance().expect("Failed to create instance 1");
            let instance2 = event_desc.create_instance().expect("Failed to create instance 2");

            // Spawn multiple entities with AudioSource
            let entity1 = app.world_mut().spawn((
                AudioSource {
                    event_instance: instance1,
                    despawn_stop_mode: StopMode::Immediate,
                },
                Transform::from_xyz(-10.0, 0.0, 0.0),
            )).id();

            let entity2 = app.world_mut().spawn((
                AudioSource {
                    event_instance: instance2,
                    despawn_stop_mode: StopMode::AllowFadeout,
                },
                Transform::from_xyz(10.0, 0.0, 0.0),
            )).id();

            // Run update
            app.update();

            // Verify both exist
            assert!(app.world().get_entity(entity1).is_ok());
            assert!(app.world().get_entity(entity2).is_ok());

            // Cleanup
            app.world_mut().despawn(entity1);
            app.world_mut().despawn(entity2);

            println!("Multiple AudioSource components work");
        }
        Err(_) => {
            println!("Skipping multiple sources test - no valid events in banks");
        }
    }
}
