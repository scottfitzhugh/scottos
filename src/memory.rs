use x86_64::{
	structures::paging::{
		FrameAllocator, PhysFrame, Size4KiB,
	},
	PhysAddr,
};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use bootloader::BootInfo;

/// Global frame allocator
pub static mut FRAME_ALLOCATOR: Option<BootInfoFrameAllocator> = None;

/// Initialize the memory management system from BootInfo (simplified)
pub fn init(boot_info: &'static BootInfo) {
	let frame_allocator = unsafe {
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};

	// Store frame allocator globally
	unsafe {
		FRAME_ALLOCATOR = Some(frame_allocator);
	}
}

/// Frame allocator that returns usable frames from the bootloader's memory map
pub struct BootInfoFrameAllocator {
	memory_map: &'static MemoryMap,
	next: usize,
}

impl BootInfoFrameAllocator {
	/// Create a new frame allocator from the memory map
	pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
		BootInfoFrameAllocator {
			memory_map,
			next: 0,
		}
	}

	/// Returns an iterator over the usable frames specified in the memory map
	fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
		let regions = self.memory_map.iter();
		let usable_regions = regions
			.filter(|r| r.region_type == MemoryRegionType::Usable);
		let addr_ranges = usable_regions
			.map(|r| r.range.start_addr()..r.range.end_addr());
		let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
		frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
	}
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
	fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
		let frame = self.usable_frames().nth(self.next);
		self.next += 1;
		frame
	}
} 