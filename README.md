# ScottOS - A Minimalist POSIX-Compliant Operating System

ScottOS is a minimalist operating system written in Rust that aims to be fully POSIX-compliant while maintaining simplicity and clarity in its design. This OS is built from scratch using modern systems programming practices and leverages Rust's memory safety features to prevent common OS vulnerabilities.

## Features

### Core Features
- **Memory Management**: Virtual memory with paging support
- **Process Management**: Preemptive multitasking with round-robin scheduling
- **Interrupt Handling**: Complete interrupt descriptor table with hardware interrupt support
- **File System**: In-memory file system with POSIX-like directory structure
- **System Calls**: POSIX-compliant system call interface
- **Async Tasks**: Cooperative multitasking for kernel tasks

### POSIX Compliance
- Standard system calls (read, write, open, close, etc.)
- Process management (fork, exec, wait, exit)
- File operations and directory management
- Standard error codes and return values
- UNIX-like file system hierarchy

### Hardware Support
- VGA text mode display
- PS/2 keyboard input
- Serial communication (for debugging)
- Timer interrupts for preemptive scheduling
- x86_64 architecture

## Architecture

### Memory Management
- **Paging**: 4-level page table implementation
- **Heap Allocation**: Linked list allocator for dynamic memory
- **Virtual Memory**: Complete virtual address space management
- **Frame Allocation**: Physical frame allocator from bootloader memory map

### Process Management
- **Scheduler**: Round-robin scheduling algorithm
- **Process Control Blocks**: Full process state management
- **Context Switching**: Register save/restore for process switches
- **Process Hierarchy**: Parent-child process relationships

### File System
- **In-Memory FS**: Simple but complete file system implementation
- **POSIX Structure**: Standard directories (/bin, /etc, /home, etc.)
- **File Descriptors**: Standard file descriptor management
- **Metadata**: Complete file metadata with permissions and timestamps

### Interrupt System
- **IDT**: Complete interrupt descriptor table
- **Exception Handlers**: CPU exception handling (page faults, double faults, etc.)
- **Hardware Interrupts**: Timer and keyboard interrupt handling
- **PIC Management**: Programmable interrupt controller configuration

## Building and Running

### Prerequisites

Before you can build and run ScottOS, you need to install the following components:

#### 1. Install Rust Nightly Toolchain
```bash
# Install rustup if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart your shell or run:
source ~/.cargo/env

# Install and set nightly as default for this project
rustup toolchain install nightly
rustup default nightly

# Verify installation
rustc --version
# Should show something like: rustc 1.89.0-nightly
```

#### 2. Install Required Rust Components
```bash
# Add rust source code (required for building core library from source)
rustup component add rust-src

# Add LLVM tools (required for linking)
rustup component add llvm-tools-preview

# Verify components are installed
rustup component list --installed
```

#### 3. Install Bootimage Tool
```bash
# Install the bootimage tool for creating bootable disk images
cargo install bootimage

# Verify installation
cargo bootimage --version
# Should show: bootimage 0.10.3 (or similar)
```

#### 4. Install QEMU (for running the OS)
```bash
# On macOS:
brew install qemu

# On Ubuntu/Debian:
sudo apt update
sudo apt install qemu-system-x86

# On Arch Linux:
sudo pacman -S qemu

# On Windows:
# Download and install QEMU from https://www.qemu.org/download/

# Verify installation
qemu-system-x86_64 --version
```

### Step-by-Step Build Instructions

Open a new terminal window and follow these steps:

#### 1. Clone and Navigate to the Project
```bash
# Navigate to the project directory
cd /path/to/scottos

# Verify you're in the right directory
ls -la
# You should see Cargo.toml, src/, x86_64-scottos.json, etc.
```

#### 2. Clean Build (Optional but Recommended)
```bash
# Clean any previous build artifacts
cargo clean

# This removes the target/ directory and ensures a fresh build
```

#### 3. Build the Kernel
```bash
# Build the ScottOS kernel
cargo build

# You should see output like:
#   Compiling scottos v0.1.0 (/path/to/scottos)
#   Finished `dev` profile [unoptimized + debuginfo] target(s) in X.XXs
```

#### 4. Create Bootable Image
```bash
# Create a bootable disk image
cargo bootimage

# You should see output like:
#   Building kernel
#   Finished `dev` profile [optimized + debuginfo] target(s) in X.XXs
#   Building bootloader
#   Compiling bootloader v0.9.31
#   Finished `release` profile [optimized + debuginfo] target(s) in X.XXs
#   Created bootimage for `scottos` at `/path/to/target/x86_64-scottos/debug/bootimage-scottos.bin`
```

#### 5. Run ScottOS in QEMU
```bash
# Method 1: Use cargo run (easiest)
cargo run

# Method 2: Run QEMU directly with more control
qemu-system-x86_64 -drive format=raw,file=target/x86_64-scottos/debug/bootimage-scottos.bin

# Method 3: Run with additional QEMU options (debugging)
qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64-scottos/debug/bootimage-scottos.bin \
  -serial stdio \
  -display curses
```

### What You Should See

When ScottOS boots successfully, you should see output similar to:

```
ScottOS v0.1.0
Initializing kernel...
GDT initialized
Interrupts initialized
Memory management initialized
File system initialized
Process scheduler initialized
ScottOS initialization complete!
System ready for operation
Async task running!
```

### Testing the System

#### 1. Keyboard Input
- Type on your keyboard - characters should appear on screen
- The async keyboard system processes input asynchronously

#### 2. System Calls (when implemented)
- Various POSIX system calls are stubbed out for future implementation

#### 3. Run Kernel Tests
```bash
# Run the kernel test suite
cargo test

# This will run tests in QEMU and exit automatically
```

### Troubleshooting

#### Common Issues and Solutions

**Issue**: `can't find crate for 'core'`
```bash
# Solution: Ensure rust-src is installed
rustup component add rust-src
```

**Issue**: `rust-lld not found`
```bash
# Solution: Install LLVM tools
rustup component add llvm-tools-preview
```

**Issue**: `bootimage not found`
```bash
# Solution: Install bootimage tool
cargo install bootimage
```

**Issue**: QEMU not starting or black screen
```bash
# Solution 1: Try with serial output
qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64-scottos/debug/bootimage-scottos.bin \
  -serial stdio

# Solution 2: Enable VGA text mode
qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64-scottos/debug/bootimage-scottos.bin \
  -vga std
```

**Issue**: Build warnings about unused code
```bash
# These are normal for a development OS - the code is ready for future features
# You can ignore warnings or build with:
cargo build --quiet
```

### Development Workflow

#### Quick Development Cycle
```bash
# 1. Make code changes
# 2. Build and run in one command
cargo run

# 3. Or for just building without running
cargo build

# 4. Run tests
cargo test
```

#### Debugging Tips
```bash
# Enable serial output for debugging
cargo run -- -serial stdio

# Build in release mode (faster but less debug info)
cargo build --release
cargo bootimage --release

# View build artifacts
ls -la target/x86_64-scottos/debug/
```

### Advanced Usage

#### Custom QEMU Options
```bash
# Run with custom memory size
qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64-scottos/debug/bootimage-scottos.bin \
  -m 512M

# Enable KVM acceleration (Linux only)
qemu-system-x86_64 \
  -enable-kvm \
  -drive format=raw,file=target/x86_64-scottos/debug/bootimage-scottos.bin

# Run in background
qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64-scottos/debug/bootimage-scottos.bin \
  -nographic \
  -serial stdio &
```

#### Cleaning Up
```bash
# Remove all build artifacts
cargo clean

# Remove bootimage cache
rm -rf ~/.cargo/registry/cache/

# Full clean rebuild
cargo clean && cargo build && cargo bootimage && cargo run
```

### Next Steps

Once you have ScottOS running:
1. **Explore the code**: Check out the modular architecture in `src/`
2. **Add features**: Implement additional system calls or device drivers  
3. **Test thoroughly**: Use `cargo test` to run the test suite
4. **Experiment**: Try modifying the kernel and see the results

Remember: Press `Ctrl+C` in the terminal to stop QEMU and return to your shell.

## Code Structure

```
src/
├── main.rs              # Kernel entry point and initialization
├── lib.rs               # Library exports and common functions
├── vga_buffer.rs        # VGA text mode display driver
├── serial.rs            # Serial communication for debugging
├── gdt.rs               # Global Descriptor Table setup
├── interrupts.rs        # Interrupt handling and IDT
├── memory.rs            # Memory management and paging
├── allocator.rs         # Heap allocator implementation
├── syscall.rs           # System call interface
├── fs.rs                # File system implementation
├── process.rs           # Process management and scheduling
├── keyboard.rs          # Keyboard input handling
└── task/                # Async task system
    ├── mod.rs           # Task definitions
    ├── executor.rs      # Task executor
    └── keyboard.rs      # Async keyboard processing
```

## System Calls

ScottOS implements standard POSIX system calls:

### Process Management
- `fork()` - Create child process
- `exec()` - Execute program
- `exit()` - Terminate process
- `wait()` - Wait for child process
- `getpid()` - Get process ID

### File Operations
- `open()` - Open file
- `close()` - Close file
- `read()` - Read from file
- `write()` - Write to file
- `lseek()` - Seek in file

### Directory Operations
- `mkdir()` - Create directory
- `rmdir()` - Remove directory
- `readdir()` - Read directory contents

### System Information
- `uname()` - Get system information
- `stat()` - Get file status
- `access()` - Check file access permissions

## Development Philosophy

### Memory Safety
ScottOS leverages Rust's ownership system and borrow checker to prevent:
- Buffer overflows
- Use-after-free vulnerabilities
- Double-free errors
- Memory leaks
- Race conditions

### Modularity
The system is designed with clear separation of concerns:
- Each module has a specific responsibility
- Well-defined interfaces between components
- Minimal coupling between subsystems
- Comprehensive error handling

### POSIX Compliance
ScottOS aims for full POSIX compliance to ensure:
- Compatibility with standard UNIX tools
- Familiar programming interface
- Standard behavior and semantics
- Portability of applications

## Contributing

This is an educational project demonstrating OS development principles. Key areas for contribution:

1. **Extended System Calls**: Implement more POSIX system calls
2. **Device Drivers**: Add support for more hardware devices
3. **File Systems**: Implement persistent file system formats
4. **Networking**: Add TCP/IP stack and network drivers
5. **User Space**: Develop shell and basic utilities

## Future Enhancements

### Short Term
- [ ] Complete system call implementation
- [ ] Add more device drivers (mouse, disk)
- [ ] Implement persistent file system
- [ ] Add shell and basic utilities

### Long Term
- [ ] SMP (Symmetric Multiprocessing) support
- [ ] Advanced memory management (COW, swapping)
- [ ] Network stack implementation
- [ ] Graphics support
- [ ] Package management system

## License

This project is educational and demonstrates OS development concepts. Feel free to use and modify for learning purposes.

## References

- [OSDev Wiki](https://wiki.osdev.org/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Writing an OS in Rust](https://os.phil-opp.com/)
- [POSIX.1-2017 Standard](https://pubs.opengroup.org/onlinepubs/9699919799/)

---

**ScottOS** - A minimalist, educational operating system showcasing modern systems programming with Rust. 