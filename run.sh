#!/bin/bash
# ScottOS Run Script
# This script runs ScottOS in QEMU

# Ensure we're using the nightly toolchain
export PATH="$HOME/.cargo/bin:$PATH"

# Change to script directory
cd "$(dirname "$0")"

echo "Building ScottOS..."
cargo build || exit 1

echo "Creating bootable image..."
cargo bootimage || exit 1

echo ""
echo "Starting ScottOS in QEMU..."
echo "Press Ctrl+A then X to exit QEMU"
echo "Or close the window to quit"
echo ""

# Run QEMU with VGA display
qemu-system-x86_64 \
	-drive format=raw,file=target/x86_64-scottos/debug/bootimage-scottos.bin \
	-serial stdio \
	-m 128M

