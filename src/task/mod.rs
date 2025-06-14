use core::{future::Future, pin::Pin, task::{Context, Poll}};
use alloc::boxed::Box;

pub mod executor;
pub mod keyboard;

pub use executor::Executor;

/// Unique task identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TaskId(u64);

impl TaskId {
	fn new() -> Self {
		use core::sync::atomic::{AtomicU64, Ordering};
		static NEXT_ID: AtomicU64 = AtomicU64::new(0);
		TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
	}
}

/// A cooperative task with a unique ID
pub struct Task {
	pub(crate) id: TaskId,
	pub(crate) future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
	/// Create a new Task with the given future
	pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
		Task {
			id: TaskId::new(),
			future: Box::pin(future),
		}
	}

	/// Poll the task and return whether it's ready
	pub(crate) fn poll(&mut self, context: &mut Context) -> Poll<()> {
		self.future.as_mut().poll(context)
	}
} 