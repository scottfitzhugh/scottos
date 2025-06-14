use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::{println, gdt, hlt_loop};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;

/// Offset for PIC interrupts
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

/// Chained PICs for handling hardware interrupts
pub static PICS: spin::Mutex<ChainedPics> =
	spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

/// Hardware interrupt numbers
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
	Timer = PIC_1_OFFSET,
	Keyboard,
}

impl InterruptIndex {
	fn as_u8(self) -> u8 {
		self as u8
	}

	fn as_usize(self) -> usize {
		usize::from(self.as_u8())
	}
}

lazy_static! {
	/// Interrupt Descriptor Table with handlers for CPU exceptions and hardware interrupts
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		
		// CPU Exception handlers
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		unsafe {
			idt.double_fault.set_handler_fn(double_fault_handler)
				.set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
		}
		idt.page_fault.set_handler_fn(page_fault_handler);
		
		// Hardware interrupt handlers
		idt[InterruptIndex::Timer.as_usize()]
			.set_handler_fn(timer_interrupt_handler);
		idt[InterruptIndex::Keyboard.as_usize()]
			.set_handler_fn(keyboard_interrupt_handler);
		
		idt
	};
}

/// Initialize the Interrupt Descriptor Table
pub fn init_idt() {
	IDT.load();
}

/// Breakpoint exception handler
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
	println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// Double fault exception handler - critical system error
extern "x86-interrupt" fn double_fault_handler(
	stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
	panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

/// Page fault exception handler
extern "x86-interrupt" fn page_fault_handler(
	stack_frame: InterruptStackFrame,
	error_code: PageFaultErrorCode,
) {
	use x86_64::registers::control::Cr2;

	println!("EXCEPTION: PAGE FAULT");
	println!("Accessed Address: {:?}", Cr2::read());
	println!("Error Code: {:?}", error_code);
	println!("{:#?}", stack_frame);
	hlt_loop();
}

/// Timer interrupt handler for preemptive multitasking
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
	// TODO: Implement process scheduling here
	unsafe {
		PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
	}
}

/// Keyboard interrupt handler
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
	use x86_64::instructions::port::Port;

	let mut port = Port::new(0x60);
	let scancode: u8 = unsafe { port.read() };
	
	// Add scancode to async processing queue
	crate::task::keyboard::add_scancode(scancode);

	unsafe {
		PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
	}
}

/// Test for breakpoint exception
#[test_case]
fn test_breakpoint_exception() {
	// Invoke a breakpoint exception to test the handler
	x86_64::instructions::interrupts::int3();
} 