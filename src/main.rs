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

/// Extremely minimal kernel entry point
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
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

/// Minimal panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	// Write "PANIC" to VGA buffer
	let vga_buffer = 0xb8000 as *mut u8;
	unsafe {
		*vga_buffer.offset(160) = b'P'; // Second line
		*vga_buffer.offset(161) = 0x4f; // Red background
		*vga_buffer.offset(162) = b'A';
		*vga_buffer.offset(163) = 0x4f;
		*vga_buffer.offset(164) = b'N';
		*vga_buffer.offset(165) = 0x4f;
		*vga_buffer.offset(166) = b'I';
		*vga_buffer.offset(167) = 0x4f;
		*vga_buffer.offset(168) = b'C';
		*vga_buffer.offset(169) = 0x4f;
	}
	
	loop {
		unsafe {
			core::arch::asm!("hlt");
		}
	}
}

// Remove test runner for now to avoid any complexity
#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
	loop {}
} 