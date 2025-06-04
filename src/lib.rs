#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod allocator;
pub mod task;
pub mod keyboard;
pub mod syscall;
pub mod fs;
pub mod process;
pub mod shell;

/// Initialize the kernel
pub fn init() {
	gdt::init();
	interrupts::init_idt();
	unsafe { interrupts::PICS.lock().initialize() };
	x86_64::instructions::interrupts::enable();
}

/// Halt loop for when the kernel has nothing to do
pub fn hlt_loop() -> ! {
	loop {
		x86_64::instructions::hlt();
	}
}

/// QEMU exit codes for testing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
	Success = 0x10,
	Failed = 0x11,
}

/// Exit QEMU with given exit code
pub fn exit_qemu(exit_code: QemuExitCode) {
	use x86_64::instructions::port::Port;

	unsafe {
		let mut port = Port::new(0xf4);
		port.write(exit_code as u32);
	}
}

pub trait Testable {
	fn run(&self) -> ();
}

impl<T> Testable for T
where
	T: Fn(),
{
	fn run(&self) {
		serial_print!("{}...\t", core::any::type_name::<T>());
		self();
		serial_println!("[ok]");
	}
}

/// Test runner function
pub fn test_runner(tests: &[&dyn Testable]) {
	serial_println!("Running {} tests", tests.len());
	for test in tests {
		test.run();
	}
	exit_qemu(QemuExitCode::Success);
}

/// Test panic handler
pub fn test_panic_handler(info: &PanicInfo) -> ! {
	serial_println!("[failed]\n");
	serial_println!("Error: {}\n", info);
	exit_qemu(QemuExitCode::Failed);
	hlt_loop();
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	init();
	test_main();
	hlt_loop();
}

/// Test panic handler for lib tests
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	test_panic_handler(info)
}

#[test_case]
fn test_println() {
	println!("test_println output");
} 