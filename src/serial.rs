use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
	/// Serial port 1 for debugging output
	pub static ref SERIAL1: Mutex<SerialPort> = {
		let mut serial_port = unsafe { SerialPort::new(0x3F8) };
		serial_port.init();
		Mutex::new(serial_port)
	};
}

/// Internal print function for serial output
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
	use core::fmt::Write;
	use x86_64::instructions::interrupts;

	interrupts::without_interrupts(|| {
		SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
	});
}

/// Serial print macro - outputs to serial port
#[macro_export]
macro_rules! serial_print {
	($($arg:tt)*) => {
		$crate::serial::_print(format_args!($($arg)*));
	};
}

/// Serial println macro - outputs to serial port with newline
#[macro_export]
macro_rules! serial_println {
	() => ($crate::serial_print!("\n"));
	($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
} 