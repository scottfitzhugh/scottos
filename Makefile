# ScottOS Makefile
# Convenient commands for building and running the operating system

.PHONY: all build run test clean docs help setup

# Default target
all: build

# Build the operating system
build:
	@echo "Building ScottOS..."
	cargo build

# Build release version
build-release:
	@echo "Building ScottOS (release)..."
	cargo build --release

# Create bootable image
bootimage:
	@echo "Creating bootable image..."
	cargo bootimage

# Create release bootable image
bootimage-release:
	@echo "Creating release bootable image..."
	cargo bootimage --release

# Run the OS in QEMU
run:
	@echo "Running ScottOS in QEMU..."
	cargo run

# Run release version
run-release:
	@echo "Running ScottOS (release) in QEMU..."
	cargo run --release

# Run with KVM acceleration (Linux only)
run-kvm:
	@echo "Running ScottOS with KVM acceleration..."
	cargo run -- -enable-kvm

# Run tests
test:
	@echo "Running kernel tests..."
	cargo test

# Run tests with output
test-verbose:
	@echo "Running kernel tests (verbose)..."
	cargo test -- --nocapture

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -f target/x86_64-scottos/debug/bootimage-*.bin
	rm -f target/x86_64-scottos/release/bootimage-*.bin

# Generate documentation
docs:
	@echo "Generating documentation..."
	cargo doc --open

# Setup development environment
setup:
	@echo "Setting up development environment..."
	rustup component add rust-src
	rustup component add llvm-tools-preview
	cargo install bootimage
	@echo "Setup complete! You can now run 'make build' to build ScottOS."

# Show help
help:
	@echo "ScottOS Build System"
	@echo "==================="
	@echo ""
	@echo "Available targets:"
	@echo "  build          - Build the operating system (debug)"
	@echo "  build-release  - Build the operating system (release)"
	@echo "  bootimage      - Create bootable disk image (debug)"
	@echo "  bootimage-release - Create bootable disk image (release)"
	@echo "  run            - Run the OS in QEMU (debug)"
	@echo "  run-release    - Run the OS in QEMU (release)"
	@echo "  run-kvm        - Run the OS with KVM acceleration"
	@echo "  test           - Run kernel tests"
	@echo "  test-verbose   - Run kernel tests with verbose output"
	@echo "  clean          - Clean build artifacts"
	@echo "  docs           - Generate and open documentation"
	@echo "  setup          - Setup development environment"
	@echo "  help           - Show this help message"
	@echo ""
	@echo "Quick start:"
	@echo "  make setup     # First time setup"
	@echo "  make build     # Build the OS"
	@echo "  make run       # Run in QEMU"

# Debug targets
debug-info:
	@echo "Build information:"
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"
	@echo "Target: x86_64-scottos"
	@echo "Features: no_std, POSIX-compliant, memory-safe"

# Show binary size
size:
	@echo "Binary size information:"
	@ls -lh target/x86_64-scottos/debug/scottos 2>/dev/null || echo "Debug binary not found - run 'make build' first"
	@ls -lh target/x86_64-scottos/release/scottos 2>/dev/null || echo "Release binary not found - run 'make build-release' first"

# Profile the build
profile:
	@echo "Building with timing information..."
	cargo build --timings

# Check code formatting
fmt-check:
	@echo "Checking code formatting..."
	cargo fmt -- --check

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Run clippy lints
clippy:
	@echo "Running Clippy lints..."
	cargo clippy

# Security audit
audit:
	@echo "Running security audit..."
	cargo audit || echo "cargo-audit not installed - run 'cargo install cargo-audit'" 