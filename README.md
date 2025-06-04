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
- Rust nightly toolchain
- `bootimage` tool: `cargo install bootimage`
- QEMU (for testing): `brew install qemu` or equivalent

### Building
```bash
# Install required components
rustup component add rust-src
rustup component add llvm-tools-preview

# Build the OS
cargo build

# Create bootable image
cargo bootimage
```

### Running in QEMU
```bash
# Run the OS in QEMU
cargo run

# Or run the bootimage directly
qemu-system-x86_64 -drive format=raw,file=target/x86_64-scottos/debug/bootimage-scottos.bin
```

### Testing
```bash
# Run kernel tests
cargo test
```

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