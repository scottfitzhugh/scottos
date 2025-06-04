#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(scottos::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use scottos::println;

entry_point!(kernel_main);

/// Main kernel entry point called by the bootloader
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
	// Immediately test VGA output without any complex initialization
	println!("=== SCOTTOS BASIC TEST ===");
	println!("If you can see this, VGA output works!");
	println!("Testing basic kernel functionality...");
	
	// Don't initialize anything complex yet - just test VGA
	println!("VGA buffer is working correctly");
	println!("Kernel entry point reached successfully");
	println!("No complex initialization - just basic output test");
	
	// Simple counter loop to show the kernel is running
	println!("Starting simple counter loop...");
	for i in 0..10 {
		println!("Counter: {}", i);
		
		// Simple delay
		for _ in 0..10000000 {
			core::hint::spin_loop();
		}
	}
	
	println!("Counter loop complete - kernel is stable!");
	println!("Entering infinite halt loop...");
	
	// Simple halt loop
	loop {
		x86_64::instructions::hlt();
	}
}

/// Test runner for kernel tests
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
	println!("Running {} tests", tests.len());
	for test in tests {
		test();
	}
	scottos::exit_qemu(scottos::QemuExitCode::Success);
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("\n!!! KERNEL PANIC !!!");
	if let Some(location) = info.location() {
		println!("Panic at {}:{}", location.file(), location.line());
	}
	println!("Panic info: {}", info);
	println!("System halted - this may cause a reboot loop");
	loop {
		x86_64::instructions::hlt();
	}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	scottos::test_panic_handler(info)
} 