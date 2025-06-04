use alloc::{string::String, vec::Vec};
use crate::{println, print, syscall};

/// Simple command-line shell for ScottOS
pub struct Shell {
	current_line: String,
	command_history: Vec<String>,
}

impl Shell {
	/// Create a new shell instance
	pub fn new() -> Self {
		Shell {
			current_line: String::new(),
			command_history: Vec::new(),
		}
	}

	/// Start the shell and display the prompt
	pub fn start(&mut self) {
		println!("\nWelcome to ScottOS Shell v0.1.0");
		println!("Type 'help' for available commands");
		self.show_prompt();
	}

	/// Process a character input from the keyboard
	pub fn process_char(&mut self, c: char) {
		match c {
			'\n' | '\r' => {
				// Enter pressed - execute command
				println!();
				if !self.current_line.is_empty() {
					self.execute_command(self.current_line.clone());
					self.command_history.push(self.current_line.clone());
					self.current_line.clear();
				}
				self.show_prompt();
			}
			'\u{8}' => {
				// Backspace pressed
				if !self.current_line.is_empty() {
					self.current_line.pop();
					print!("\u{8} \u{8}"); // Backspace, space, backspace to clear character
				}
			}
			c if c.is_ascii() && !c.is_control() => {
				// Regular ASCII character
				self.current_line.push(c);
				print!("{}", c);
			}
			_ => {
				// Ignore other characters (function keys, etc.)
			}
		}
	}

	/// Display the shell prompt
	fn show_prompt(&self) {
		print!("scottos:~$ ");
	}

	/// Execute a command entered by the user
	fn execute_command(&self, command: String) {
		let parts: Vec<&str> = command.trim().split_whitespace().collect();
		if parts.is_empty() {
			return;
		}

		let cmd = parts[0];
		let args = &parts[1..];

		match cmd {
			"help" => self.cmd_help(),
			"clear" => self.cmd_clear(),
			"echo" => self.cmd_echo(args),
			"uname" => self.cmd_uname(),
			"whoami" => self.cmd_whoami(),
			"uptime" => self.cmd_uptime(),
			"memory" => self.cmd_memory(),
			"version" => self.cmd_version(),
			"history" => self.cmd_history(),
			"exit" => self.cmd_exit(),
			"reboot" => self.cmd_reboot(),
			"syscall" => self.cmd_syscall(args),
			"test" => self.cmd_test(args),
			_ => {
				println!("Command '{}' not found. Type 'help' for available commands.", cmd);
			}
		}
	}

	/// Show help information
	fn cmd_help(&self) {
		println!("ScottOS Shell - Available Commands:");
		println!("  help      - Show this help message");
		println!("  clear     - Clear the screen");
		println!("  echo      - Echo arguments to the screen");
		println!("  uname     - Show system information");
		println!("  whoami    - Show current user");
		println!("  uptime    - Show system uptime (placeholder)");
		println!("  memory    - Show memory information (placeholder)");
		println!("  version   - Show ScottOS version");
		println!("  history   - Show command history");
		println!("  syscall   - Test system calls");
		println!("  test      - Run various tests");
		println!("  exit      - Exit the shell (halt system)");
		println!("  reboot    - Reboot the system");
	}

	/// Clear the screen
	fn cmd_clear(&self) {
		// Clear VGA buffer by printing many newlines
		for _ in 0..25 {
			println!();
		}
		println!("ScottOS v0.1.0 - Shell Cleared");
	}

	/// Echo command - print arguments
	fn cmd_echo(&self, args: &[&str]) {
		if args.is_empty() {
			println!();
		} else {
			println!("{}", args.join(" "));
		}
	}

	/// Show system information using uname syscall
	fn cmd_uname(&self) {
		let mut buffer = [0u8; 390];
		match syscall::syscall_handler(63, buffer.as_mut_ptr() as usize, 0, 0, 0, 0, 0) {
			Ok(_) => {
				// Try to parse the uname buffer
				if let Ok(info) = core::str::from_utf8(&buffer[0..7]) {
					println!("System: {}", info.trim_end_matches('\0'));
				} else {
					println!("ScottOS x86_64");
				}
			}
			Err(_) => {
				println!("ScottOS x86_64");
			}
		}
	}

	/// Show current user
	fn cmd_whoami(&self) {
		println!("root");
	}

	/// Show system uptime (placeholder)
	fn cmd_uptime(&self) {
		println!("System uptime: Running since boot (timer not implemented)");
	}

	/// Show memory information (placeholder)
	fn cmd_memory(&self) {
		println!("Memory usage: Heap allocator active (detailed stats not implemented)");
	}

	/// Show ScottOS version
	fn cmd_version(&self) {
		println!("ScottOS v0.1.0 - A minimalist POSIX-compliant operating system");
		println!("Built with Rust (nightly)");
		println!("Target: x86_64-scottos");
	}

	/// Show command history
	fn cmd_history(&self) {
		println!("Command history:");
		for (i, cmd) in self.command_history.iter().enumerate() {
			println!("  {}: {}", i + 1, cmd);
		}
	}

	/// Test system calls
	fn cmd_syscall(&self, args: &[&str]) {
		if args.is_empty() {
			println!("Usage: syscall <test_name>");
			println!("Available tests: getpid, write, uname");
			return;
		}

		match args[0] {
			"getpid" => {
				match syscall::syscall_handler(39, 0, 0, 0, 0, 0, 0) {
					Ok(pid) => println!("Process ID: {}", pid),
					Err(e) => println!("Error: {:?}", e),
				}
			}
			"write" => {
				let test_msg = b"Hello from syscall!\n";
				match syscall::syscall_handler(1, 1, test_msg.as_ptr() as usize, test_msg.len(), 0, 0, 0) {
					Ok(bytes) => println!("Wrote {} bytes", bytes),
					Err(e) => println!("Error: {:?}", e),
				}
			}
			"uname" => {
				self.cmd_uname();
			}
			_ => {
				println!("Unknown syscall test: {}", args[0]);
			}
		}
	}

	/// Run various tests
	fn cmd_test(&self, args: &[&str]) {
		if args.is_empty() {
			println!("Available tests: keyboard, interrupts, memory");
			return;
		}

		match args[0] {
			"keyboard" => {
				println!("Keyboard test: Type some characters, they should appear on screen");
			}
			"interrupts" => {
				println!("Testing interrupts...");
				// Trigger a breakpoint to test interrupt handling
				x86_64::instructions::interrupts::int3();
				println!("Breakpoint interrupt handled successfully!");
			}
			"memory" => {
				println!("Testing memory allocation...");
				let test_vec = Vec::<u32>::with_capacity(100);
				println!("Allocated vector with capacity: {}", test_vec.capacity());
			}
			_ => {
				println!("Unknown test: {}", args[0]);
			}
		}
	}

	/// Exit the shell (halt the system)
	fn cmd_exit(&self) {
		println!("Shutting down ScottOS...");
		println!("Thank you for using ScottOS!");
		crate::hlt_loop();
	}

	/// Reboot the system
	fn cmd_reboot(&self) {
		println!("Rebooting ScottOS...");
		// For now, just halt - real reboot would require more complex implementation
		crate::hlt_loop();
	}
}

/// Global shell instance for async keyboard processing
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
	pub static ref SHELL: Mutex<Shell> = Mutex::new(Shell::new());
}

/// Initialize the shell system
pub fn init_shell() {
	SHELL.lock().start();
} 