use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use core::{pin::Pin, task::{Poll, Context}};
use futures_util::stream::{Stream, StreamExt};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use crate::{println, print};

/// Keyboard scancode queue
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

/// Called by the keyboard interrupt handler
/// Must not block or allocate.
pub(crate) fn add_scancode(scancode: u8) {
	if let Ok(queue) = SCANCODE_QUEUE.try_get() {
		if let Err(_) = queue.push(scancode) {
			println!("WARNING: scancode queue full; dropping keyboard input");
		}
	} else {
		println!("WARNING: scancode queue uninitialized");
	}
}

/// Scancode stream for async keyboard processing
pub struct ScancodeStream {
	_private: (),
}

impl ScancodeStream {
	pub fn new() -> Self {
		SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
			.expect("ScancodeStream::new should only be called once");
		ScancodeStream { _private: () }
	}
}

impl Stream for ScancodeStream {
	type Item = u8;

	fn poll_next(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<u8>> {
		let queue = SCANCODE_QUEUE.try_get().expect("scancode queue not initialized");

		// fast path
		if let Ok(scancode) = queue.pop() {
			return Poll::Ready(Some(scancode));
		}

		// TODO: register waker for keyboard interrupt
		Poll::Pending
	}
}

/// Async task for printing keypresses
pub async fn print_keypresses() {
	let mut scancodes = ScancodeStream::new();
	let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
		HandleControl::Ignore);

	while let Some(scancode) = scancodes.next().await {
		if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
			if let Some(key) = keyboard.process_keyevent(key_event) {
				match key {
					DecodedKey::Unicode(character) => print!("{}", character),
					DecodedKey::RawKey(key) => print!("{:?}", key),
				}
			}
		}
	}
} 