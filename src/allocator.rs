use linked_list_allocator::LockedHeap;

/// Global heap allocator instance
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Heap start address
pub const HEAP_START: usize = 0x_4444_4444_0000;
/// Heap size in bytes (1 MB)
pub const HEAP_SIZE: usize = 1024 * 1024;

/// Initialize the heap allocator (simplified version)
pub fn init_heap() -> Result<(), &'static str> {
	// For now, use a simple heap initialization without complex memory mapping
	// This is a simplified approach for getting the shell working
	unsafe {
		// Note: This is a simplified approach that assumes the bootloader
		// has already set up basic memory management. In a full implementation,
		// you would need proper page table setup.
		ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
	}

	Ok(())
}

/// Allocation error handler
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
	panic!("allocation error: {:?}", layout)
} 