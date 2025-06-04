use alloc::{collections::BTreeMap, vec::Vec, string::String};
use alloc::string::ToString;
use spin::Mutex;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Process identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessId(pub usize);

impl ProcessId {
	/// Create a new unique process ID
	fn new() -> Self {
		static NEXT_PID: AtomicUsize = AtomicUsize::new(1);
		ProcessId(NEXT_PID.fetch_add(1, Ordering::Relaxed))
	}
}

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
	Ready,
	Running,
	Blocked,
	Terminated,
}

/// Process control block
#[derive(Debug, Clone)]
pub struct Process {
	pub pid: ProcessId,
	pub parent_pid: Option<ProcessId>,
	pub state: ProcessState,
	pub name: String,
	pub priority: u8,
	pub memory_base: usize,
	pub memory_size: usize,
	pub registers: ProcessRegisters,
	pub open_files: Vec<usize>, // File descriptors
}

/// Saved process registers
#[derive(Debug, Clone, Copy)]
pub struct ProcessRegisters {
	pub rax: u64,
	pub rbx: u64,
	pub rcx: u64,
	pub rdx: u64,
	pub rsi: u64,
	pub rdi: u64,
	pub rbp: u64,
	pub rsp: u64,
	pub r8: u64,
	pub r9: u64,
	pub r10: u64,
	pub r11: u64,
	pub r12: u64,
	pub r13: u64,
	pub r14: u64,
	pub r15: u64,
	pub rip: u64,
	pub rflags: u64,
}

impl Default for ProcessRegisters {
	fn default() -> Self {
		ProcessRegisters {
			rax: 0, rbx: 0, rcx: 0, rdx: 0,
			rsi: 0, rdi: 0, rbp: 0, rsp: 0,
			r8: 0, r9: 0, r10: 0, r11: 0,
			r12: 0, r13: 0, r14: 0, r15: 0,
			rip: 0, rflags: 0x202, // Enable interrupts
		}
	}
}

impl Process {
	/// Create a new process
	pub fn new(name: String, parent_pid: Option<ProcessId>) -> Self {
		Process {
			pid: ProcessId::new(),
			parent_pid,
			state: ProcessState::Ready,
			name,
			priority: 100, // Default priority
			memory_base: 0,
			memory_size: 0,
			registers: ProcessRegisters::default(),
			open_files: Vec::new(),
		}
	}

	/// Set process as running
	pub fn set_running(&mut self) {
		self.state = ProcessState::Running;
	}

	/// Set process as ready
	pub fn set_ready(&mut self) {
		self.state = ProcessState::Ready;
	}

	/// Set process as blocked
	pub fn set_blocked(&mut self) {
		self.state = ProcessState::Blocked;
	}

	/// Terminate the process
	pub fn terminate(&mut self) {
		self.state = ProcessState::Terminated;
	}
}

/// Process scheduler
pub struct Scheduler {
	processes: BTreeMap<ProcessId, Process>,
	ready_queue: Vec<ProcessId>,
	current_process: Option<ProcessId>,
	time_slice: usize,
	current_time_slice: usize,
}

impl Scheduler {
	/// Create a new scheduler
	pub fn new() -> Self {
		Scheduler {
			processes: BTreeMap::new(),
			ready_queue: Vec::new(),
			current_process: None,
			time_slice: 10, // Time slice in timer ticks
			current_time_slice: 0,
		}
	}

	/// Add a new process to the scheduler
	pub fn add_process(&mut self, process: Process) {
		let pid = process.pid;
		self.processes.insert(pid, process);
		self.ready_queue.push(pid);
	}

	/// Get the current running process
	pub fn current_process(&self) -> Option<&Process> {
		self.current_process.and_then(|pid| self.processes.get(&pid))
	}

	/// Get the current running process mutably
	pub fn current_process_mut(&mut self) -> Option<&mut Process> {
		self.current_process.and_then(move |pid| self.processes.get_mut(&pid))
	}

	/// Schedule the next process to run
	pub fn schedule(&mut self) -> Option<ProcessId> {
		// Simple round-robin scheduling
		if let Some(current_pid) = self.current_process {
			// Move current process back to ready queue if still ready
			if let Some(process) = self.processes.get_mut(&current_pid) {
				if process.state == ProcessState::Running {
					process.set_ready();
					self.ready_queue.push(current_pid);
				}
			}
		}

		// Get next process from ready queue
		while let Some(pid) = self.ready_queue.pop() {
			if let Some(process) = self.processes.get_mut(&pid) {
				if process.state == ProcessState::Ready {
					process.set_running();
					self.current_process = Some(pid);
					self.current_time_slice = self.time_slice;
					return Some(pid);
				}
			}
		}

		self.current_process = None;
		None
	}

	/// Handle timer tick for preemptive scheduling
	pub fn timer_tick(&mut self) {
		if self.current_time_slice > 0 {
			self.current_time_slice -= 1;
		}
		
		// Force context switch if time slice expired
		if self.current_time_slice == 0 {
			self.schedule();
		}
	}

	/// Remove a process from the scheduler
	pub fn remove_process(&mut self, pid: ProcessId) {
		if let Some(mut process) = self.processes.remove(&pid) {
			process.terminate();
			
			// Remove from ready queue
			self.ready_queue.retain(|&p| p != pid);
			
			// If this was the current process, schedule next
			if self.current_process == Some(pid) {
				self.current_process = None;
				self.schedule();
			}
		}
	}

	/// Get process by PID
	pub fn get_process(&self, pid: ProcessId) -> Option<&Process> {
		self.processes.get(&pid)
	}

	/// Get process by PID mutably
	pub fn get_process_mut(&mut self, pid: ProcessId) -> Option<&mut Process> {
		self.processes.get_mut(&pid)
	}

	/// List all processes
	pub fn list_processes(&self) -> Vec<&Process> {
		self.processes.values().collect()
	}
}

/// Global process scheduler
static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler {
	processes: BTreeMap::new(),
	ready_queue: Vec::new(),
	current_process: None,
	time_slice: 10,
	current_time_slice: 0,
});

/// Initialize the process management system
pub fn init() {
	let mut scheduler = SCHEDULER.lock();
	
	// Create init process (PID 1)
	let init_process = Process::new("init".to_string(), None);
	scheduler.add_process(init_process);
	
	// Start scheduling
	scheduler.schedule();
}

/// Initialize the process scheduler with the init process
pub fn init_scheduler() {
	with_scheduler(|scheduler| {
		// Create the init process (PID 1)
		let init_process = Process::new("init".to_string(), None);
		scheduler.add_process(init_process);
	});
}

/// Execute a function with access to the global scheduler
pub fn with_scheduler<F, R>(f: F) -> R
where
	F: FnOnce(&mut Scheduler) -> R,
{
	f(&mut SCHEDULER.lock())
}

/// Get the current process ID
pub fn current_pid() -> Option<ProcessId> {
	SCHEDULER.lock().current_process
}

/// Create a new process
pub fn spawn_process(name: String, parent_pid: Option<ProcessId>) -> ProcessId {
	let process = Process::new(name, parent_pid);
	let pid = process.pid;
	
	SCHEDULER.lock().add_process(process);
	pid
}

/// Terminate a process
pub fn terminate_process(pid: ProcessId) {
	SCHEDULER.lock().remove_process(pid);
}

/// Handle timer interrupt for scheduling
pub fn handle_timer_interrupt() {
	SCHEDULER.lock().timer_tick();
} 