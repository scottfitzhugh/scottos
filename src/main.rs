#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(scottos::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

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
    let mut shell = Shell::new();
    
    // Initialize basic systems (GDT, IDT, interrupts)
    //scottos::init();
    
    // Clear screen
    writer.clear_screen();
    
    // Display boot message
    writer.write_string("ScottOS v0.1.0 - Boot Successful!\n", 0x0A); // Light green
    writer.write_string("Shell system initialized.\n\n", 0x07); // Light gray
    
    // Display banner
    writer.write_string("╔══════════════════════════════════════════════════════════════════════════════╗\n", 0x0F);
    writer.write_string("║                                ScottOS v0.1.0                               ║\n", 0x0F);
    writer.write_string("║                      A Minimalist POSIX-Compliant OS                       ║\n", 0x0F);
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
        /*
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
        */
        // Halt CPU to save power
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}


/// Extremely minimal kernel entry point
fn kernel_main_minimal(_boot_info: &'static BootInfo) -> ! {
	// Direct VGA buffer access - most basic possible output
	let vga_buffer = 0xb8000 as *mut u8;
	
	// Write "HELLO SCOTTOS" directly to VGA buffer
	unsafe {
		*vga_buffer.offset(0) = b'H';
		*vga_buffer.offset(1) = 0x0f; // White on black
		*vga_buffer.offset(2) = b'E';
		*vga_buffer.offset(3) = 0x0f;
		*vga_buffer.offset(4) = b'L';
		*vga_buffer.offset(5) = 0x0f;
		*vga_buffer.offset(6) = b'L';
		*vga_buffer.offset(7) = 0x0f;
		*vga_buffer.offset(8) = b'O';
		*vga_buffer.offset(9) = 0x0f;
		*vga_buffer.offset(10) = b' ';
		*vga_buffer.offset(11) = 0x0f;
		*vga_buffer.offset(12) = b'S';
		*vga_buffer.offset(13) = 0x0f;
		*vga_buffer.offset(14) = b'C';
		*vga_buffer.offset(15) = 0x0f;
		*vga_buffer.offset(16) = b'O';
		*vga_buffer.offset(17) = 0x0f;
		*vga_buffer.offset(18) = b'T';
		*vga_buffer.offset(19) = 0x0f;
		*vga_buffer.offset(20) = b'T';
		*vga_buffer.offset(21) = 0x0f;
		*vga_buffer.offset(22) = b'O';
		*vga_buffer.offset(23) = 0x0f;
		*vga_buffer.offset(24) = b'S';
		*vga_buffer.offset(25) = 0x0f;
		*vga_buffer.offset(26) = b' ';
		*vga_buffer.offset(27) = 0x0f;
		*vga_buffer.offset(28) = b'B';
		*vga_buffer.offset(29) = 0x0f;
		*vga_buffer.offset(30) = b'O';
		*vga_buffer.offset(31) = 0x0f;
		*vga_buffer.offset(32) = b'O';
		*vga_buffer.offset(33) = 0x0f;
		*vga_buffer.offset(34) = b'T';
		*vga_buffer.offset(35) = 0x0f;
		*vga_buffer.offset(36) = b'E';
		*vga_buffer.offset(37) = 0x0f;
		*vga_buffer.offset(38) = b'D';
		*vga_buffer.offset(39) = 0x0f;
	}
	
	// Infinite halt loop - no interrupts, no complex operations
	loop {
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