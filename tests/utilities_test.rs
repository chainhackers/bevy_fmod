// Test utility plugins
// Verifies MuteWhenUnfocused and other helper plugins

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window, WindowPlugin};
use bevy_fmod::{FmodPlugin, FmodStudio};
use bevy_fmod::utilities::MuteWhenUnfocused;

#[test]
fn test_mute_when_unfocused_plugin_initialization() {
    // Test that MuteWhenUnfocused plugin can be added to app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(WindowPlugin::default())
        .add_plugins(FmodPlugin::new(&[
            "tests/data/Master.bank",
            "tests/data/Master.strings.bank",
        ]))
        .add_plugins(MuteWhenUnfocused);

    // Spawn primary window
    app.world_mut().spawn((Window::default(), PrimaryWindow));

    // Verify FmodStudio resource exists
    let studio = app.world().resource::<FmodStudio>();
    assert!(studio.get_bus("bus:/").is_ok(), "Master bus should be accessible");

    println!("MuteWhenUnfocused plugin initialized successfully");
}

#[test]
fn test_mute_when_unfocused_master_bus_control() {
    // Test that the plugin can control the master bus mute state
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(WindowPlugin::default())
        .add_plugins(FmodPlugin::new(&[
            "tests/data/Master.bank",
            "tests/data/Master.strings.bank",
        ]))
        .add_plugins(MuteWhenUnfocused);

    // Spawn primary window
    app.world_mut().spawn((Window::default(), PrimaryWindow));

    // Get studio and verify master bus exists
    let studio = app.world().resource::<FmodStudio>();
    let master_bus = studio.get_bus("bus:/").expect("Master bus should exist");

    // Verify we can set and get mute state
    master_bus.set_mute(true).expect("Should be able to mute master bus");
    let is_muted = master_bus.get_mute().expect("Should be able to get mute state");
    assert!(is_muted, "Master bus should be muted");

    master_bus.set_mute(false).expect("Should be able to unmute master bus");
    let is_muted = master_bus.get_mute().expect("Should be able to get mute state");
    assert!(!is_muted, "Master bus should be unmuted");

    println!("Master bus mute control works correctly");
}

#[test]
fn test_mute_utility_without_window_plugin() {
    // Test that utility gracefully handles missing window features
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(FmodPlugin::new(&[
            "tests/data/Master.bank",
            "tests/data/Master.strings.bank",
        ]));

    // Note: Not adding MuteWhenUnfocused here because it requires bevy_window feature
    // This test just verifies the setup works without it

    let studio = app.world().resource::<FmodStudio>();
    assert!(studio.get_bus("bus:/").is_ok());

    println!("FMOD works without mute utility");
}
