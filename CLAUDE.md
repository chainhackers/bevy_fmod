# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`bevy_fmod` is a Bevy plugin that provides idiomatic integration with the FMOD audio engine. It wraps the `libfmod` crate and provides a Bevy-native API for spatial audio, event handling, and live updates.

Current version: 0.9.0 (compatible with Bevy 0.17)

## Build and Test Commands

### Building
```bash
cargo build
cargo build --release
```

### Running Examples
Examples require the FMOD demo project to be set up (see README.md for details):
```bash
cargo run --example minimal
cargo run --example audio_control
cargo run --example spatial
cargo run --example parameters
```

### Live Update Feature
Enable live update to connect FMOD Studio during runtime:
```bash
cargo run --example minimal --features live-update
```

### Testing

**Recommended Method (uses helper script):**
```bash
./run_tests.sh
```

The helper script automatically sets up FMOD library paths and runs tests with single-threaded execution (required for FMOD audio system).

**Manual Testing:**
```bash
# Set FMOD SDK path
export FMOD_SDK_DIR=/path/to/fmodstudioapi20310linux

# Set runtime library path
export LD_LIBRARY_PATH=$FMOD_SDK_DIR/api/core/lib/x86_64:$FMOD_SDK_DIR/api/studio/lib/x86_64:$LD_LIBRARY_PATH

# Run tests (must be single-threaded)
cargo test -- --test-threads=1
```

**Alternative: Configure .cargo/config.toml**
```bash
# Copy the template
cp .cargo/config.toml.example .cargo/config.toml

# Edit to match your FMOD installation paths
# Then run tests normally
cargo test -- --test-threads=1
```

**Important Notes:**
- Tests **must** run single-threaded due to FMOD audio system constraints
- FMOD libraries must be accessible at both compile-time (linking) and runtime (LD_LIBRARY_PATH)
- Some tests are marked with `#[ignore]` and require actual FMOD bank files

### Linting
The project enforces strict linting:
```bash
cargo clippy
```

## Platform-Specific Setup

### Linux
Set the `FMOD_SDK_DIR` environment variable before building:
```bash
export FMOD_SDK_DIR=/path/to/fmod/20309_processed
export LD_LIBRARY_PATH=$FMOD_SDK_DIR/api/core/lib/x86_64:$FMOD_SDK_DIR/api/studio/lib/x86_64:$LD_LIBRARY_PATH
```

The build script (build.rs) handles library linking using this environment variable or falls back to system paths.

### macOS
Libraries should be placed in `vendor/` directory. The `.cargo/config.toml` configures linking:
- Uses `-L native=./vendor` for library search path
- Sets rpath to `./vendor` for runtime library resolution

### Windows
FMOD DLLs must be renamed with `_vc` suffix:
- `fmod.dll` → `fmod_vc.dll`
- `fmodstudio.dll` → `fmodstudio_vc.dll`

Place these in the project root for development, alongside the executable for distribution.

## Code Architecture

### Core Components

**FmodPlugin** (`src/fmod_plugin.rs`)
- Main Bevy plugin that initializes FMOD Studio
- Requires paths to FMOD bank files
- Optionally loads custom FMOD plugins
- Registers component hooks for cleanup
- Updates audio sources and listeners each frame

**FmodStudio** (`src/fmod_studio.rs`)
- Resource wrapping the FMOD Studio API
- Initializes with right-handed 3D coordinates
- Loads banks and plugins at startup
- Provides access to event descriptions and system

**Components** (`src/components/`)
- `AudioSource`: Attached to entities that emit sound, wraps FMOD event instances
- `AudioListener`: Marks entities as audio listeners (typically cameras)
- `Velocity`: Tracks entity velocity for Doppler effect
- Bundles for convenient entity setup

**Utilities** (`src/utilities/`)
- `MuteWhenUnfocused`: Plugin that automatically mutes audio when the primary window loses focus

### Plugin Lifecycle

1. **PreStartup**: Component hooks registered for cleanup
2. **Update**: 3D attributes updated for sources and listeners
3. **PostUpdate**: FMOD Studio system update called

### Component Hooks

AudioSource components automatically clean up on removal:
- Stops event instance (with configured stop mode)
- Releases FMOD resources

## Code Style (STYLEGUIDE.md)

### Imports
- NO wildcard imports in library code (enforced by `#![deny(clippy::wildcard_imports)]`)
- Direct imports from `bevy::prelude` allowed
- Wildcard imports from `bevy::prelude::*` allowed in examples only

### Access Modifiers
- Everything private by default
- Use `pub(crate)` for internal cross-module access
- Export user-facing types through `prelude` module
- Re-export in standard module hierarchy

Example pattern:
```rust
// lib.rs
pub mod fmod_studio;

// fmod_studio.rs
pub struct FmodStudioPlugin;

// prelude.rs
pub use crate::fmod_studio::FmodStudioPlugin;
```

### Documentation
The crate uses `#![deny(missing_docs)]` - all public items must be documented.

## Dependencies

- `libfmod`: Path dependency to `../libfmod/libfmod`
- `bevy`: Version 0.17 (minimal features in lib, full features in dev)

## Common Patterns

### Creating an Audio Source
```rust
let event_description = studio.get_event("event:/Music/Level 03").unwrap();
commands.spawn(AudioSource {
    event_instance: event_description.create_instance().unwrap(),
    despawn_stop_mode: StopMode::AllowFadeout,
});
```

### Plugin Initialization
```rust
FmodPlugin::new(&[
    "./assets/audio/demo_project/Build/Desktop/Master.bank",
    "./assets/audio/demo_project/Build/Desktop/Master.strings.bank",
])
```

## Features

- `utilities` (default): Enables utility plugins like MuteWhenUnfocused (requires bevy_window)
- `live-update`: Enables FMOD Studio live connection (development only)

## Git Commit Guidelines

- Always add files individually using their full paths (never use `git add -A` or `git add -u`)
- Keep commit messages concise and on a single line
- Use Conventional Commits format (https://www.conventionalcommits.org/en/v1.0.0/)
    - Common prefixes: `feat:`, `fix:`, `docs:`, `style:`, `refactor:`, `test:`, `chore:`
    - For CI/CD and devops: use `chore(ci):` prefix
- Add issue/task number at the end of commit messages (e.g., `feat: add futures trading support #1`)
- Do NOT add attribution lines or Co-Authored-By to commits
- Do NOT use emoji in commit messages
- Avoid redundant adjectives in commit messages, test names, and documentation:
    - Don't use: "comprehensive", "robust", "sophisticated", "advanced", "complete", "critical", "important"
    - These words are redundant - tests should be thorough by default, bugs are important by nature
    - Calling a test "comprehensive" implies other tests are shallow
    - Calling a fix "critical" implies other fixes aren't important
    - Good names are specific: "test_parallel_subscriptions", "test_reconnection", "test_error_handling"
- Focus on what changed, not how well it was done

## Notes

- FMOD libraries are NOT included due to licensing - users must download separately
- The crate re-exports `libfmod` for plugin authors: `pub use libfmod;`
- Uses FMOD 2.03.09 (as of latest upgrade)
- Spatial audio uses right-handed 3D coordinate system (`FMOD_INIT_3D_RIGHTHANDED`)
