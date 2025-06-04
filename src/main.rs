#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(scottos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

/// VGA text buffer constants
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

/// Simple shell constants
const MAX_COMMAND_LEN: usize = 64;
const MAX_HISTORY: usize = 10;

/// VGA colors
#[allow(dead_code)]
#[repr(u8)]
enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// Simple VGA writer
struct VgaWriter {
    row: usize,
    col: usize,
}

impl VgaWriter {
    fn new() -> Self {
        Self { row: 0, col: 0 }
    }
    
    fn write_byte(&mut self, byte: u8, color: u8) {
        if byte == b'\n' {
            self.new_line();
            return;
        }
        
        if self.col >= VGA_WIDTH {
            self.new_line();
        }
        
        let offset = (self.row * VGA_WIDTH + self.col) * 2;
        unsafe {
            *VGA_BUFFER.offset(offset as isize) = byte;
            *VGA_BUFFER.offset(offset as isize + 1) = color;
        }
        self.col += 1;
    }
    
    fn write_string(&mut self, s: &str, color: u8) {
        for byte in s.bytes() {
            self.write_byte(byte, color);
        }
    }
    
    fn new_line(&mut self) {
        self.row += 1;
        self.col = 0;
        if self.row >= VGA_HEIGHT {
            self.scroll();
        }
    }
    
    fn scroll(&mut self) {
        // Simple scroll - move all lines up
        unsafe {
            for row in 1..VGA_HEIGHT {
                for col in 0..VGA_WIDTH {
                    let src_offset = (row * VGA_WIDTH + col) * 2;
                    let dst_offset = ((row - 1) * VGA_WIDTH + col) * 2;
                    *VGA_BUFFER.offset(dst_offset as isize) = *VGA_BUFFER.offset(src_offset as isize);
                    *VGA_BUFFER.offset(dst_offset as isize + 1) = *VGA_BUFFER.offset(src_offset as isize + 1);
                }
            }
            // Clear last line
            for col in 0..VGA_WIDTH {
                let offset = ((VGA_HEIGHT - 1) * VGA_WIDTH + col) * 2;
                *VGA_BUFFER.offset(offset as isize) = b' ';
                *VGA_BUFFER.offset(offset as isize + 1) = 0x07;
            }
        }
        self.row = VGA_HEIGHT - 1;
        self.col = 0;
    }
    
    fn clear_screen(&mut self) {
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            unsafe {
                *VGA_BUFFER.offset(i as isize * 2) = b' ';
                *VGA_BUFFER.offset(i as isize * 2 + 1) = 0x07; // Light gray on black
            }
        }
        self.row = 0;
        self.col = 0;
    }
    
    fn backspace(&mut self) {
        if self.col > 0 {
            self.col -= 1;
            let offset = (self.row * VGA_WIDTH + self.col) * 2;
            unsafe {
                *VGA_BUFFER.offset(offset as isize) = b' ';
                *VGA_BUFFER.offset(offset as isize + 1) = 0x07;
            }
        }
    }
}

/// Simple shell state
struct SimpleShell {
    current_line: [u8; MAX_COMMAND_LEN],
    current_len: usize,
    command_history: [[u8; MAX_COMMAND_LEN]; MAX_HISTORY],
    history_count: usize,
}

impl SimpleShell {
    fn new() -> Self {
        Self {
            current_line: [0; MAX_COMMAND_LEN],
            current_len: 0,
            command_history: [[0; MAX_COMMAND_LEN]; MAX_HISTORY],
            history_count: 0,
        }
    }
    
    fn add_char(&mut self, c: u8) -> bool {
        if self.current_len < MAX_COMMAND_LEN - 1 {
            self.current_line[self.current_len] = c;
            self.current_len += 1;
            true
        } else {
            false
        }
    }
    
    fn backspace(&mut self) -> bool {
        if self.current_len > 0 {
            self.current_len -= 1;
            self.current_line[self.current_len] = 0;
            true
        } else {
            false
        }
    }
    
    fn execute_command(&mut self, writer: &mut VgaWriter) {
        // Save command to history
        if self.current_len > 0 && self.history_count < MAX_HISTORY {
            for i in 0..self.current_len {
                self.command_history[self.history_count][i] = self.current_line[i];
            }
            // Null terminate
            if self.current_len < MAX_COMMAND_LEN {
                self.command_history[self.history_count][self.current_len] = 0;
            }
            self.history_count += 1;
        }
        
        // Convert command to string slice for processing
        let command_str = core::str::from_utf8(&self.current_line[..self.current_len]).unwrap_or("");
        
        writer.new_line();
        
        // Process command
        match command_str.trim() {
            "" => {
                // Empty command
            }
            "help" => {
                writer.write_string("Available commands:\n", 0x0A);
                writer.write_string("  help     - Show this help message\n", 0x07);
                writer.write_string("  clear    - Clear the screen\n", 0x07);
                writer.write_string("  echo     - Echo text back\n", 0x07);
                writer.write_string("  uname    - System information\n", 0x07);
                writer.write_string("  whoami   - Current user\n", 0x07);
                writer.write_string("  uptime   - System uptime\n", 0x07);
                writer.write_string("  version  - OS version\n", 0x07);
                writer.write_string("  history  - Command history\n", 0x07);
                writer.write_string("  reboot   - Restart system\n", 0x07);
            }
            "clear" => {
                writer.clear_screen();
            }
            "uname" => {
                writer.write_string("ScottOS v0.1.0 x86_64\n", 0x0A);
            }
            "whoami" => {
                writer.write_string("root\n", 0x0A);
            }
            "uptime" => {
                writer.write_string("System has been running since boot\n", 0x0A);
            }
            "version" => {
                writer.write_string("ScottOS v0.1.0\n", 0x0A);
                writer.write_string("Built with Rust - A Minimalist POSIX-Compliant OS\n", 0x07);
            }
            "history" => {
                writer.write_string("Command history:\n", 0x0A);
                for i in 0..self.history_count {
                    writer.write_string("  ", 0x07);
                    // Find null terminator or end of command
                    let mut len = 0;
                    for j in 0..MAX_COMMAND_LEN {
                        if self.command_history[i][j] == 0 {
                            break;
                        }
                        len += 1;
                    }
                    let cmd_str = core::str::from_utf8(&self.command_history[i][..len]).unwrap_or("");
                    writer.write_string(cmd_str, 0x0F);
                    writer.write_string("\n", 0x07);
                }
            }
            "reboot" => {
                writer.write_string("Rebooting system...\n", 0x0C);
                // Simple reboot via keyboard controller
                unsafe {
                    let port = 0x64u16;
                    core::arch::asm!("out dx, al", in("dx") port, in("al") 0xFEu8);
                }
            }
            cmd if cmd.starts_with("echo ") => {
                let text = &cmd[5..]; // Skip "echo "
                writer.write_string(text, 0x0A);
                writer.write_string("\n", 0x07);
            }
            _ => {
                writer.write_string("Command not found: ", 0x0C);
                writer.write_string(command_str, 0x0F);
                writer.write_string("\n", 0x07);
                writer.write_string("Type 'help' for available commands.\n", 0x08);
            }
        }
        
        // Clear current command
        self.current_len = 0;
        for i in 0..MAX_COMMAND_LEN {
            self.current_line[i] = 0;
        }
        
        // Show new prompt
        writer.write_string("scottos:~$ ", 0x0B);
    }
}

/// Read a scancode from the keyboard
fn read_scancode() -> Option<u8> {
    use x86_64::instructions::port::Port;
    
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    
    // Check if there's data available
    let mut status_port = Port::new(0x64);
    let status: u8 = unsafe { status_port.read() };
    
    if status & 0x01 != 0 {
        Some(scancode)
    } else {
        None
    }
}

/// Convert scancode to ASCII (simple US layout)
fn scancode_to_ascii(scancode: u8) -> Option<u8> {
    match scancode {
        0x02 => Some(b'1'), 0x03 => Some(b'2'), 0x04 => Some(b'3'), 0x05 => Some(b'4'),
        0x06 => Some(b'5'), 0x07 => Some(b'6'), 0x08 => Some(b'7'), 0x09 => Some(b'8'),
        0x0A => Some(b'9'), 0x0B => Some(b'0'),
        0x10 => Some(b'q'), 0x11 => Some(b'w'), 0x12 => Some(b'e'), 0x13 => Some(b'r'),
        0x14 => Some(b't'), 0x15 => Some(b'y'), 0x16 => Some(b'u'), 0x17 => Some(b'i'),
        0x18 => Some(b'o'), 0x19 => Some(b'p'),
        0x1E => Some(b'a'), 0x1F => Some(b's'), 0x20 => Some(b'd'), 0x21 => Some(b'f'),
        0x22 => Some(b'g'), 0x23 => Some(b'h'), 0x24 => Some(b'j'), 0x25 => Some(b'k'),
        0x26 => Some(b'l'),
        0x2C => Some(b'z'), 0x2D => Some(b'x'), 0x2E => Some(b'c'), 0x2F => Some(b'v'),
        0x30 => Some(b'b'), 0x31 => Some(b'n'), 0x32 => Some(b'm'),
        0x39 => Some(b' '), // Space
        0x0C => Some(b'-'), 0x0D => Some(b'='),
        0x33 => Some(b','), 0x34 => Some(b'.'), 0x35 => Some(b'/'),
        _ => None,
    }
}

/// Working kernel with shell command prompt
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    let mut writer = VgaWriter::new();
    let mut shell = SimpleShell::new();
    
    // Initialize basic systems (GDT, IDT, interrupts)
    scottos::init();
    
    // Clear screen
    writer.clear_screen();
    
    // Display boot message
    writer.write_string("ScottOS v0.1.0 - Boot Successful!\n", 0x0A); // Light green
    writer.write_string("Shell system initialized.\n\n", 0x07); // Light gray
    
    // Display banner
    writer.write_string("╔══════════════════════════════════════════════════════════════════════════════╗\n", 0x0F);
    writer.write_string("║                                ScottOS v0.1.0                               ║\n", 0x0F);
    writer.write_string("║                      A Minimalist POSIX-Compliant OS                       ║\n", 0x0F);
    writer.write_string("║                              Built with Rust                               ║\n", 0x0F);
    writer.write_string("╚══════════════════════════════════════════════════════════════════════════════╝\n", 0x0F);
    writer.write_string("\n", 0x07);
    
    // Display welcome message
    writer.write_string("Welcome to ScottOS!\n", 0x0E); // Yellow
    writer.write_string("Interactive shell ready. Type 'help' for available commands.\n", 0x07);
    writer.write_string("\n", 0x07);
    
    // Display initial shell prompt
    writer.write_string("scottos:~$ ", 0x0B); // Light cyan
    
    // Main shell loop
    loop {
        // Check for keyboard input
        if let Some(scancode) = read_scancode() {
            match scancode {
                0x0E => { // Backspace
                    if shell.backspace() {
                        writer.backspace();
                    }
                }
                0x1C => { // Enter
                    shell.execute_command(&mut writer);
                }
                code => {
                    // Try to convert to ASCII
                    if let Some(ascii) = scancode_to_ascii(code) {
                        if shell.add_char(ascii) {
                            writer.write_byte(ascii, 0x0F);
                        }
                    }
                }
            }
        }
        
        // Halt CPU to save power
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Test runner for kernel tests
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    // Can't use println! here, so just halt
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Panic handler - shows error information and halts
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = VgaWriter::new();
    
    // Clear screen and show panic
    writer.clear_screen();
    writer.write_string("╔══════════════════════════════════════════════════════════════════════════════╗\n", 0x4F);
    writer.write_string("║                               KERNEL PANIC                                  ║\n", 0x4F);
    writer.write_string("╚══════════════════════════════════════════════════════════════════════════════╝\n", 0x4F);
    writer.write_string("\n", 0x07);
    
    if let Some(location) = info.location() {
        writer.write_string("Panic at: ", 0x0C);
        writer.write_string(location.file(), 0x0F);
        writer.write_string(":", 0x0F);
        writer.write_string("\n", 0x07);
    }
    
    writer.write_string("\nSystem halted due to panic.\n", 0x0C);
    writer.write_string("Reboot required.\n", 0x08);
    
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
} 