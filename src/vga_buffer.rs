use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

/// Standard VGA colors
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}

/// Color code combining foreground and background colors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
	/// Create new color code with foreground and background colors
	fn new(foreground: Color, background: Color) -> ColorCode {
		ColorCode((background as u8) << 4 | (foreground as u8))
	}
}

/// VGA character with color information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
	ascii_character: u8,
	color_code: ColorCode,
}

/// VGA text buffer dimensions
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// VGA text buffer structure
#[repr(transparent)]
struct Buffer {
	chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// VGA writer for managing text output
pub struct Writer {
	column_position: usize,
	color_code: ColorCode,
	buffer: &'static mut Buffer,
}

impl Writer {
	/// Write a single byte to the VGA buffer
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.column_position >= BUFFER_WIDTH {
					self.new_line();
				}

				let row = BUFFER_HEIGHT - 1;
				let col = self.column_position;

				let color_code = self.color_code;
				self.buffer.chars[row][col].write(ScreenChar {
					ascii_character: byte,
					color_code,
				});
				self.column_position += 1;
			}
		}
	}

	/// Write a string to the VGA buffer
	pub fn write_string(&mut self, s: &str) {
		for byte in s.bytes() {
			match byte {
				// Printable ASCII byte or newline
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				// Not part of printable ASCII range
				_ => self.write_byte(0xfe),
			}
		}
	}

	/// Create a new line by scrolling and moving cursor
	fn new_line(&mut self) {
		for row in 1..BUFFER_HEIGHT {
			for col in 0..BUFFER_WIDTH {
				let character = self.buffer.chars[row][col].read();
				self.buffer.chars[row - 1][col].write(character);
			}
		}
		self.clear_row(BUFFER_HEIGHT - 1);
		self.column_position = 0;
	}

	/// Clear a specific row
	fn clear_row(&mut self, row: usize) {
		let blank = ScreenChar {
			ascii_character: b' ',
			color_code: self.color_code,
		};
		for col in 0..BUFFER_WIDTH {
			self.buffer.chars[row][col].write(blank);
		}
	}
}

impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

lazy_static! {
	/// Global VGA writer instance
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
		column_position: 0,
		color_code: ColorCode::new(Color::Yellow, Color::Black),
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
	});
}

/// Print macro implementation
#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

/// Println macro implementation
#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Internal print function
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	use x86_64::instructions::interrupts;

	interrupts::without_interrupts(|| {
		WRITER.lock().write_fmt(args).unwrap();
	});
}

/// Test for VGA buffer basic functionality
#[test_case]
fn test_println_simple() {
	println!("test_println_simple output");
}

/// Test for VGA buffer output integrity
#[test_case]
fn test_println_many() {
	for _ in 0..200 {
		println!("test_println_many output");
	}
}

/// Test that println output appears on screen
#[test_case]
fn test_println_output() {
	use core::fmt::Write;
	use x86_64::instructions::interrupts;

	let s = "Some test string that fits on a single line";
	interrupts::without_interrupts(|| {
		let mut writer = WRITER.lock();
		writeln!(writer, "\n{}", s).expect("writeln failed");
		for (i, c) in s.chars().enumerate() {
			let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
			assert_eq!(char::from(screen_char.ascii_character), c);
		}
	});
} 