use linked_list_allocator::LockedHeap;

/// Global heap allocator instance
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Heap size in bytes (100 KB - enough for shell and basic operations)
pub const HEAP_SIZE: usize = 100 * 1024;

/// Static heap buffer in the BSS section
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

/// Initialize the heap allocator using a static buffer
pub fn init_heap() -> Result<(), &'static str> {
	unsafe {
		let heap_start = HEAP.as_ptr() as usize;
		ALLOCATOR.lock().init(heap_start, HEAP_SIZE);
	}

	Ok(())
}

/// Allocation error handler
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
	panic!("allocation error: {:?}", layout)
} 