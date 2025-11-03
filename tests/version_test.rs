// Test FMOD version compatibility
use libfmod::System;

#[test]
fn test_fmod_version() {
    let system = System::create().expect("Failed to create FMOD system");
    let (version, build) = system.get_version().expect("Failed to get FMOD version");

    // FMOD version format: 0xAAAABBCC where AAAA=major, BB=minor, CC=patch
    let major = (version >> 16) & 0xFFFF;
    let minor = (version >> 8) & 0xFF;
    let patch = version & 0xFF;

    println!("FMOD Version: {}.{:02}.{:02} (build {})", major, minor, patch, build);

    // Verify we're using FMOD 2.03.x (version 0x00020300 - 0x000203FF)
    assert_eq!(major, 2, "Expected FMOD major version 2");
    assert_eq!(minor, 3, "Expected FMOD minor version 03");

    system.release().ok();
}

#[test]
fn test_fmod_system_info() {
    let system = System::create().expect("Failed to create FMOD system");

    // Get driver count
    let driver_count = system.get_num_drivers().expect("Failed to get driver count");
    println!("Available audio drivers: {}", driver_count);

    // This test should pass even with 0 drivers (headless systems)
    assert!(driver_count >= 0);

    system.release().ok();
}
