use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use lazy_static::lazy_static;

/// Double fault stack index in the TSS
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
	/// Task State Segment for handling interrupts
	static ref TSS: TaskStateSegment = {
		let mut tss = TaskStateSegment::new();
		tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
			const STACK_SIZE: usize = 4096 * 5;
			static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

			let stack_start = VirtAddr::from_ptr(&raw const STACK);
			let stack_end = stack_start + (STACK_SIZE as u64);
			stack_end
		};
		tss
	};
}

lazy_static! {
	/// Global Descriptor Table with kernel code segment and TSS
	static ref GDT: (GlobalDescriptorTable, Selectors) = {
		let mut gdt = GlobalDescriptorTable::new();
		let code_selector = gdt.append(Descriptor::kernel_code_segment());
		let tss_selector = gdt.append(Descriptor::tss_segment(&TSS));
		(gdt, Selectors { code_selector, tss_selector })
	};
}

/// Segment selectors for GDT entries
struct Selectors {
	code_selector: SegmentSelector,
	tss_selector: SegmentSelector,
}

/// Initialize the Global Descriptor Table
pub fn init() {
	use x86_64::instructions::segmentation::{CS, Segment};
	use x86_64::instructions::tables::load_tss;

	GDT.0.load();
	unsafe {
		CS::set_reg(GDT.1.code_selector);
		load_tss(GDT.1.tss_selector);
	}
} 