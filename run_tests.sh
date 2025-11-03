#!/bin/bash
# Test runner for bevy_fmod
# Sets up FMOD library paths and runs tests with proper configuration

set -e

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}bevy_fmod Test Runner${NC}"
echo "================================"

# Detect the script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Check if test banks are set up
if [ ! -e "tests/data" ]; then
    echo -e "${YELLOW}Warning: Test banks not found${NC}"
    echo "Run ./setup_tests.sh first to set up test environment"
    echo
    read -p "Run setup now? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        ./setup_tests.sh
    else
        echo -e "${RED}Cannot run tests without test banks${NC}"
        exit 1
    fi
fi

# Set FMOD SDK directory (relative to this repo) if not already set
if [ -z "$FMOD_SDK_DIR" ]; then
    export FMOD_SDK_DIR="../libfmod/fmodstudioapi20310linux"
fi

# Check if FMOD SDK exists
if [ ! -d "$FMOD_SDK_DIR" ]; then
    echo -e "${RED}Error: FMOD SDK not found at $FMOD_SDK_DIR${NC}"
    echo "Please ensure the FMOD SDK is installed at the expected location."
    echo "You can override the location by setting FMOD_SDK_DIR environment variable."
    echo
    echo "Run ./setup_tests.sh for more information."
    exit 1
fi

# Detect architecture
ARCH=$(uname -m)
if [ "$ARCH" = "aarch64" ]; then
    LIB_ARCH="arm64"
else
    LIB_ARCH="x86_64"
fi

# Set library paths
FMOD_CORE_LIB="$FMOD_SDK_DIR/api/core/lib/$LIB_ARCH"
FMOD_STUDIO_LIB="$FMOD_SDK_DIR/api/studio/lib/$LIB_ARCH"

# Verify library directories exist
if [ ! -d "$FMOD_CORE_LIB" ] || [ ! -d "$FMOD_STUDIO_LIB" ]; then
    echo -e "${RED}Error: FMOD libraries not found${NC}"
    echo "Expected core lib: $FMOD_CORE_LIB"
    echo "Expected studio lib: $FMOD_STUDIO_LIB"
    exit 1
fi

# Verify actual library files exist
if [ ! -f "$FMOD_CORE_LIB/libfmod.so.14" ] || [ ! -f "$FMOD_STUDIO_LIB/libfmodstudio.so.14" ]; then
    echo -e "${RED}Error: FMOD library files not found${NC}"
    echo "Expected files:"
    echo "  $FMOD_CORE_LIB/libfmod.so.14"
    echo "  $FMOD_STUDIO_LIB/libfmodstudio.so.14"
    exit 1
fi

# Set runtime library path
export LD_LIBRARY_PATH="$FMOD_CORE_LIB:$FMOD_STUDIO_LIB:$LD_LIBRARY_PATH"

echo -e "${GREEN}FMOD SDK:${NC} $FMOD_SDK_DIR"
echo -e "${GREEN}Architecture:${NC} $LIB_ARCH"
echo ""

# Verify FMOD works by running verification example
echo -e "${YELLOW}Verifying FMOD...${NC}"
cargo run --example verify_fmod 2>&1 | grep -v "warning:"

if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo -e "${RED}Error: FMOD verification failed${NC}"
    echo "FMOD libraries may not be properly installed or accessible."
    exit 1
fi

echo ""

# Run tests with single thread (required for FMOD audio system)
echo -e "${YELLOW}Running tests (single-threaded)...${NC}"
cargo test "$@" -- --test-threads=1

echo -e "${GREEN}Tests completed!${NC}"
