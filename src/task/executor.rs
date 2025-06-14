use super::{Task, TaskId};
use alloc::{collections::BTreeMap, sync::Arc, task::Wake};
use core::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;

/// Simple task executor for cooperative multitasking
pub struct Executor {
	tasks: BTreeMap<TaskId, Task>,
	task_queue: Arc<ArrayQueue<TaskId>>,
	waker_cache: BTreeMap<TaskId, Waker>,
}

impl Executor {
	/// Create a new executor
	pub fn new() -> Self {
		Executor {
			tasks: BTreeMap::new(),
			task_queue: Arc::new(ArrayQueue::new(100)),
			waker_cache: BTreeMap::new(),
		}
	}

	/// Spawn a new task
	pub fn spawn(&mut self, task: Task) {
		let task_id = task.id;
		if self.tasks.insert(task.id, task).is_some() {
			panic!("task with same ID already in tasks");
		}
		self.task_queue.push(task_id).expect("queue full");
	}

	/// Run all tasks to completion
	pub fn run(&mut self) -> ! {
		loop {
			self.run_ready_tasks();
			self.sleep_if_idle();
		}
	}

	/// Run all ready tasks
	fn run_ready_tasks(&mut self) {
		// destructure `self` to avoid borrow checker errors
		let Self {
			tasks,
			task_queue,
			waker_cache,
		} = self;

		while let Ok(task_id) = task_queue.pop() {
			let task = match tasks.get_mut(&task_id) {
				Some(task) => task,
				None => continue, // task no longer exists
			};
			let waker = waker_cache
				.entry(task_id)
				.or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
			let mut context = Context::from_waker(waker);
			match task.poll(&mut context) {
				Poll::Ready(()) => {
					// task done -> remove it and its cached waker
					tasks.remove(&task_id);
					waker_cache.remove(&task_id);
				}
				Poll::Pending => {}
			}
		}
	}

	/// Put the CPU to sleep if no tasks are runnable
	fn sleep_if_idle(&self) {
		use x86_64::instructions::interrupts::{self, enable_and_hlt};

		interrupts::disable();
		if self.task_queue.is_empty() {
			enable_and_hlt();
		} else {
			interrupts::enable();
		}
	}
}

/// A waker that wakes a task by pushing its ID to the task queue
struct TaskWaker {
	task_id: TaskId,
	task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
	/// Create a new TaskWaker
	fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
		Waker::from(Arc::new(TaskWaker {
			task_id,
			task_queue,
		}))
	}

	/// Wake the task by pushing it to the task queue
	fn wake_task(&self) {
		self.task_queue.push(self.task_id).expect("task_queue full");
	}
}

impl Wake for TaskWaker {
	fn wake(self: Arc<Self>) {
		self.wake_task();
	}

	fn wake_by_ref(self: &Arc<Self>) {
		self.wake_task();
	}
} 