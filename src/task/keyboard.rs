use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use core::{pin::Pin, task::{Poll, Context}};
use futures_util::stream::{Stream, StreamExt};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use crate::{println, print};
use futures_util::task::AtomicWaker;

/// Keyboard scancode queue
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

/// Called by the keyboard interrupt handler
/// Must not block or allocate.
pub(crate) fn add_scancode(scancode: u8) {
	if let Ok(queue) = SCANCODE_QUEUE.try_get() {
		if let Err(_) = queue.push(scancode) {
			println!("WARNING: scancode queue full; dropping keyboard input");
		} else {
			// Wake any task waiting on keyboard input
			WAKER.wake();
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

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
		let queue = SCANCODE_QUEUE.try_get().expect("scancode queue not initialized");

		// fast path
		if let Ok(scancode) = queue.pop() {
			return Poll::Ready(Some(scancode));
		}

		// register waker and check again to avoid race conditions
		WAKER.register(cx.waker());
		if let Ok(scancode) = queue.pop() {
			return Poll::Ready(Some(scancode));
		}

		Poll::Pending
	}
}

/// Async task for processing keypresses through the shell
pub async fn process_shell_input() {
	let mut scancodes = ScancodeStream::new();
	let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
		HandleControl::Ignore);

	while let Some(scancode) = scancodes.next().await {
		if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
			if let Some(key) = keyboard.process_keyevent(key_event) {
				match key {
					DecodedKey::Unicode(character) => {
						// Send character to shell for processing
						crate::shell::SHELL.lock().process_char(character);
					}
					DecodedKey::RawKey(_key) => {
						// For now, ignore raw keys like function keys
						// Could be extended to handle special keys
					}
				}
			}
		}
	}
}

/// Async task for printing keypresses (legacy - kept for compatibility)
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