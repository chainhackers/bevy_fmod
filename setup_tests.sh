#!/bin/bash

# Setup script for bevy_fmod tests
# Creates symlink to libfmod test banks and verifies FMOD SDK

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== bevy_fmod Test Setup ===${NC}"
echo

# Detect the script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Path to libfmod test banks (relative to bevy_fmod)
LIBFMOD_TEST_BANKS="../libfmod/libfmod/tests/data/Build/Desktop"

echo -e "${YELLOW}Checking for libfmod test banks...${NC}"

# Check if libfmod test banks exist
if [ ! -d "$LIBFMOD_TEST_BANKS" ]; then
    echo -e "${RED}✗ libfmod test banks not found${NC}"
    echo
    echo "Expected location: $LIBFMOD_TEST_BANKS"
    echo
    echo "bevy_fmod tests require FMOD Studio bank files from libfmod."
    echo "Please ensure libfmod is checked out in a sibling directory:"
    echo
    echo "  cd .."
    echo "  git clone https://github.com/chainhackers/libfmod"
    echo "  cd bevy_fmod"
    echo
    exit 1
fi

echo -e "${GREEN}✓ Found libfmod test banks${NC}"

# Create tests directory if it doesn't exist
mkdir -p tests

# Create or update symlink
if [ -L "tests/data" ]; then
    echo -e "${YELLOW}tests/data symlink already exists${NC}"
    # Verify it points to the right place
    CURRENT_TARGET=$(readlink "tests/data")
    EXPECTED_TARGET="../../libfmod/libfmod/tests/data/Build/Desktop"
    if [ "$CURRENT_TARGET" = "$EXPECTED_TARGET" ]; then
        echo -e "${GREEN}✓ Symlink is correct${NC}"
    else
        echo -e "${YELLOW}⚠ Updating symlink target${NC}"
        rm "tests/data"
        ln -s "$EXPECTED_TARGET" "tests/data"
        echo -e "${GREEN}✓ Symlink updated${NC}"
    fi
elif [ -e "tests/data" ]; then
    echo -e "${RED}✗ tests/data exists but is not a symlink${NC}"
    echo "Please remove or rename it manually, then run this script again."
    exit 1
else
    echo -n "Creating tests/data symlink... "
    ln -s "../../libfmod/libfmod/tests/data/Build/Desktop" "tests/data"
    echo -e "${GREEN}✓${NC}"
fi

# List available bank files
echo
echo "Available test banks:"
ls -lh tests/data/*.bank 2>/dev/null || echo "No bank files found"

echo
echo -e "${YELLOW}Checking for FMOD SDK...${NC}"

# Set default FMOD SDK path if not already set
if [ -z "$FMOD_SDK_DIR" ]; then
    # Try to use the one from libfmod directory
    DEFAULT_SDK="../libfmod/fmodstudioapi20310linux"
    if [ -d "$DEFAULT_SDK" ]; then
        export FMOD_SDK_DIR="$DEFAULT_SDK"
        echo -e "${YELLOW}Using FMOD SDK from libfmod: $FMOD_SDK_DIR${NC}"
    fi
fi

# Check if FMOD SDK path is set
if [ -z "$FMOD_SDK_DIR" ]; then
    echo -e "${RED}❌ FMOD_SDK_DIR not set${NC}"
    echo
    echo "Please download FMOD Engine SDK from: https://www.fmod.com/download"
    echo "Then set the environment variable:"
    echo "  export FMOD_SDK_DIR=/path/to/fmod/sdk"
    echo
    echo "Example:"
    echo "  export FMOD_SDK_DIR=/home/\$USER/fmod/fmodstudioapi20310linux"
    echo
    echo "Or place the SDK in ../libfmod/fmodstudioapi20310linux"
else
    echo -e "${GREEN}✓ FMOD_SDK_DIR is set to: $FMOD_SDK_DIR${NC}"

    # Detect architecture
    ARCH=$(uname -m)
    if [ "$ARCH" = "aarch64" ]; then
        LIB_ARCH="arm64"
    else
        LIB_ARCH="x86_64"
    fi

    # Check if the libraries exist
    if [ -d "$FMOD_SDK_DIR/api/core/lib/$LIB_ARCH" ]; then
        echo -e "${GREEN}✓ FMOD libraries found ($LIB_ARCH)${NC}"
    else
        echo -e "${RED}✗ FMOD libraries not found at expected location${NC}"
        echo "  Expected: $FMOD_SDK_DIR/api/core/lib/$LIB_ARCH"
    fi
fi

echo
echo -e "${GREEN}Setup complete!${NC}"
echo
echo "To run tests:"
echo "  chmod +x run_tests.sh"
echo "  ./run_tests.sh"
echo
echo "Or manually:"
echo "  export FMOD_SDK_DIR=/path/to/fmod/sdk"
echo "  cargo test -- --test-threads=1"
