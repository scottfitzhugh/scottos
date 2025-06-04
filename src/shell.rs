use crate::{println, print};

/// Maximum command line length
const MAX_COMMAND_LEN: usize = 256;
/// Maximum number of command history entries
const MAX_HISTORY: usize = 10;

/// Simple command-line shell for ScottOS (no dynamic allocation)
pub struct Shell {
	current_line: [u8; MAX_COMMAND_LEN],
	current_pos: usize,
	command_history: [[u8; MAX_COMMAND_LEN]; MAX_HISTORY],
	history_count: usize,
}

impl Shell {
	/// Create a new shell instance
	pub fn new() -> Self {
		Shell {
			current_line: [0; MAX_COMMAND_LEN],
			current_pos: 0,
			command_history: [[0; MAX_COMMAND_LEN]; MAX_HISTORY],
			history_count: 0,
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
				if self.current_pos > 0 {
					// Convert current line to string slice
					let command_str = core::str::from_utf8(&self.current_line[..self.current_pos])
						.unwrap_or("");
					self.execute_command(command_str);
					
					// Add to history
					if self.history_count < MAX_HISTORY {
						self.command_history[self.history_count][..self.current_pos]
							.copy_from_slice(&self.current_line[..self.current_pos]);
						// Clear the rest
						for i in self.current_pos..MAX_COMMAND_LEN {
							self.command_history[self.history_count][i] = 0;
						}
						self.history_count += 1;
					}
					
					// Clear current line
					self.current_line = [0; MAX_COMMAND_LEN];
					self.current_pos = 0;
				}
				self.show_prompt();
			}
			'\u{8}' => {
				// Backspace pressed
				if self.current_pos > 0 {
					self.current_pos -= 1;
					self.current_line[self.current_pos] = 0;
					print!("\u{8} \u{8}"); // Backspace, space, backspace to clear character
				}
			}
			c if c.is_ascii() && !c.is_control() && self.current_pos < MAX_COMMAND_LEN - 1 => {
				// Regular ASCII character
				self.current_line[self.current_pos] = c as u8;
				self.current_pos += 1;
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
	fn execute_command(&self, command: &str) {
		let command = command.trim();
		if command.is_empty() {
			return;
		}

		// Simple command parsing - split on first space
		let mut parts = command.split_whitespace();
		let cmd = parts.next().unwrap_or("");
		let args: &str = command.get(cmd.len()..).unwrap_or("").trim();

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
	fn cmd_echo(&self, args: &str) {
		if args.is_empty() {
			println!();
		} else {
			println!("{}", args);
		}
	}

	/// Show system information
	fn cmd_uname(&self) {
		println!("ScottOS x86_64");
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
		println!("Memory usage: Basic allocator active (detailed stats not implemented)");
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
		for i in 0..self.history_count {
			let cmd_bytes = &self.command_history[i];
			// Find the end of the command (first null byte)
			let mut len = 0;
			for &byte in cmd_bytes {
				if byte == 0 { break; }
				len += 1;
			}
			if len > 0 {
				if let Ok(cmd_str) = core::str::from_utf8(&cmd_bytes[..len]) {
					println!("  {}: {}", i + 1, cmd_str);
				}
			}
		}
	}

	/// Run various tests
	fn cmd_test(&self, args: &str) {
		let args = args.trim();
		if args.is_empty() {
			println!("Available tests: keyboard, interrupts");
			return;
		}

		match args {
			"keyboard" => {
				println!("Keyboard test: Type some characters, they should appear on screen");
			}
			"interrupts" => {
				println!("Testing interrupts...");
				// Trigger a breakpoint to test interrupt handling
				x86_64::instructions::interrupts::int3();
				println!("Breakpoint interrupt handled successfully!");
			}
			_ => {
				println!("Unknown test: {}", args);
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