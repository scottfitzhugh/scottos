#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(scottos::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use scottos::{println, serial_println, task::Task};

entry_point!(kernel_main);

/// Main kernel entry point
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	// Simple VGA test first
	println!("ScottOS v0.1.0 - Testing VGA output");
	
	// Initialize GDT and IDT first (required for proper operation)
	println!("Initializing GDT...");
	scottos::gdt::init();
	println!("GDT OK");
	
	println!("Initializing IDT...");
	scottos::interrupts::init_idt();
	println!("IDT OK");
	
	// Now test serial
	serial_println!("Serial output working!");
	
	println!("  [1/6] GDT initialized");
	println!("  [2/6] IDT initialized");
	
	// Initialize PIC (Programmable Interrupt Controller)
	serial_println!("  [3/6] Initializing PIC...");
	println!("  [3/6] Initializing PIC...");
	unsafe { scottos::interrupts::PICS.lock().initialize() };
	
	// Initialize memory management
	serial_println!("  [4/6] Initializing memory management...");
	println!("  [4/6] Initializing memory management...");
	scottos::memory::init(boot_info);
	
	// Initialize heap allocator
	serial_println!("  [5/6] Initializing heap allocator...");
	println!("  [5/6] Initializing heap allocator...");
	scottos::allocator::init_heap()
		.expect("heap initialization failed");
	
	// Enable interrupts
	serial_println!("  [6/6] Enabling interrupts...");
	println!("  [6/6] Enabling interrupts...");
	x86_64::instructions::interrupts::enable();
	
	serial_println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
	serial_println!("║                                ScottOS v0.1.0                                ║");
	serial_println!("║                      A Minimalist POSIX-Compliant OS                        ║");
	serial_println!("╚══════════════════════════════════════════════════════════════════════════════╝");
	serial_println!();
	println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
	println!("║                                ScottOS v0.1.0                                ║");
	println!("║                      A Minimalist POSIX-Compliant OS                        ║");
	println!("╚══════════════════════════════════════════════════════════════════════════════╝");
	println!();
	
	// Initialize shell
	serial_println!("Initializing shell system...");
	println!("Initializing shell system...");
	scottos::shell::init_shell();
	
	// Create async executor
	let mut executor = scottos::task::Executor::new();
	
	// Spawn shell keyboard processing task
	executor.spawn(Task::new(scottos::task::keyboard::process_shell_input()));
	
	// Run the executor (never returns)
	serial_println!("Starting async task executor...\n");
	println!("Starting async task executor...\n");
	serial_println!("Shell is now active. Type 'help' for available commands.");
	executor.run();
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

/// Panic handler - shows error information and halts
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
	println!("║                               KERNEL PANIC                                   ║");
	println!("╚══════════════════════════════════════════════════════════════════════════════╝");
	println!();
    
    if let Some(location) = info.location() {
		println!("Panic at: {}:{}", location.file(), location.line());
	}
	
	println!("Message: {}", info.message());
	
	println!("\nSystem halted due to panic. Reboot required.");
	
	scottos::hlt_loop();
}

/// Test panic handler
#[cfg(test)]
#[panic_handler]
fn panic_test(info: &PanicInfo) -> ! {
	scottos::test_panic_handler(info)
}
