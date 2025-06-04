#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(scottos::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;
use scottos::{println, gdt, interrupts, memory, allocator, task, fs, process};

entry_point!(kernel_main);

/// Main kernel entry point called by the bootloader
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use scottos::memory::BootInfoFrameAllocator;

	println!("ScottOS v0.1.0");
	println!("Initializing kernel...");

	// Initialize GDT
	gdt::init();
	println!("GDT initialized");

	// Initialize interrupts
	interrupts::init_idt();
	unsafe { interrupts::PICS.lock().initialize() };
	x86_64::instructions::interrupts::enable();
	println!("Interrupts initialized");

	// Initialize memory management
	let phys_mem_offset = VirtAddr::new(0);  // Placeholder - bootloader 0.9 doesn't provide this directly
	let mut mapper = unsafe { memory::init(phys_mem_offset) };
	let mut frame_allocator = unsafe {
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};

	// Initialize heap allocator
	allocator::init_heap(&mut mapper, &mut frame_allocator)
		.expect("heap initialization failed");
	println!("Memory management initialized");

	// Initialize file system
	fs::init_filesystem();
	println!("File system initialized");

	// Initialize process management
	process::init_scheduler();
	println!("Process scheduler initialized");

	// Initialize task executor
	let mut executor = task::Executor::new();
	executor.spawn(task::Task::new(example_task()));
	executor.spawn(task::Task::new(
		task::keyboard::print_keypresses(),
	));

	println!("ScottOS initialization complete!");
	println!("System ready for operation");

	#[cfg(test)]
	test_main();

	executor.run();
}

/// Example async task to demonstrate the task system
async fn example_task() {
	println!("Async task running!");
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	scottos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	scottos::test_panic_handler(info)
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