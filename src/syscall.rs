use crate::{println, print, hlt_loop};

/// POSIX system call numbers
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum SyscallNumber {
	Read = 0,
	Write = 1,
	Open = 2,
	Close = 3,
	Stat = 4,
	Fstat = 5,
	Lstat = 6,
	Poll = 7,
	Lseek = 8,
	Mmap = 9,
	Mprotect = 10,
	Munmap = 11,
	Brk = 12,
	RtSigaction = 13,
	RtSigprocmask = 14,
	RtSigreturn = 15,
	Ioctl = 16,
	Pread64 = 17,
	Pwrite64 = 18,
	Readv = 19,
	Writev = 20,
	Access = 21,
	Pipe = 22,
	Select = 23,
	SchedYield = 24,
	Mremap = 25,
	Msync = 26,
	Mincore = 27,
	Madvise = 28,
	Shmget = 29,
	Shmat = 30,
	Shmctl = 31,
	Dup = 32,
	Dup2 = 33,
	Pause = 34,
	Nanosleep = 35,
	Getitimer = 36,
	Alarm = 37,
	Setitimer = 38,
	Getpid = 39,
	Sendfile = 40,
	Socket = 41,
	Connect = 42,
	Accept = 43,
	Sendto = 44,
	Recvfrom = 45,
	Sendmsg = 46,
	Recvmsg = 47,
	Shutdown = 48,
	Bind = 49,
	Listen = 50,
	Getsockname = 51,
	Getpeername = 52,
	Socketpair = 53,
	Setsockopt = 54,
	Getsockopt = 55,
	Clone = 56,
	Fork = 57,
	Vfork = 58,
	Execve = 59,
	Exit = 60,
	Wait4 = 61,
	Kill = 62,
	Uname = 63,
	Semget = 64,
	Semop = 65,
	Semctl = 66,
	Shmdt = 67,
	Msgget = 68,
	Msgsnd = 69,
	Msgrcv = 70,
	Msgctl = 71,
	Fcntl = 72,
	Flock = 73,
	Fsync = 74,
	Fdatasync = 75,
	Truncate = 76,
	Ftruncate = 77,
	Getdents = 78,
	Getcwd = 79,
	Chdir = 80,
	Fchdir = 81,
	Rename = 82,
	Mkdir = 83,
	Rmdir = 84,
	Creat = 85,
	Link = 86,
	Unlink = 87,
	Symlink = 88,
	Readlink = 89,
	Chmod = 90,
	Fchmod = 91,
	Chown = 92,
	Fchown = 93,
	Lchown = 94,
	Umask = 95,
	Gettimeofday = 96,
	Getrlimit = 97,
	Getrusage = 98,
	Sysinfo = 99,
	Times = 100,
}

/// System call error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum SyscallError {
	Success = 0,
	PermissionDenied = -1,
	NoSuchFileOrDirectory = -2,
	NoSuchProcess = -3,
	InterruptedSystemCall = -4,
	IoError = -5,
	NoSuchDeviceOrAddress = -6,
	ArgumentListTooLong = -7,
	ExecFormatError = -8,
	BadFileNumber = -9,
	NoChildProcesses = -10,
	TryAgain = -11,
	OutOfMemory = -12,
	PermissionDenied2 = -13,
	BadAddress = -14,
	BlockDeviceRequired = -15,
	DeviceOrResourceBusy = -16,
	FileExists = -17,
	CrossDeviceLink = -18,
	NoSuchDevice = -19,
	NotADirectory = -20,
	IsADirectory = -21,
	InvalidArgument = -22,
	FileTableOverflow = -23,
	TooManyOpenFiles = -24,
	NotATypewriter = -25,
	TextFileBusy = -26,
	FileTooLarge = -27,
	NoSpaceLeftOnDevice = -28,
	IllegalSeek = -29,
	ReadOnlyFileSystem = -30,
	TooManyLinks = -31,
	BrokenPipe = -32,
	MathArgumentOutOfDomain = -33,
	MathResultNotRepresentable = -34,
}

/// System call result type
pub type SyscallResult = Result<usize, SyscallError>;

/// Handle system call dispatch
pub fn syscall_handler(
	syscall_num: usize,
	arg1: usize,
	arg2: usize,
	arg3: usize,
	arg4: usize,
	arg5: usize,
	arg6: usize,
) -> SyscallResult {
	match syscall_num {
		0 => sys_read(arg1, arg2 as *mut u8, arg3),
		1 => sys_write(arg1, arg2 as *const u8, arg3),
		2 => sys_open(arg1 as *const u8, arg2, arg3),
		3 => sys_close(arg1),
		39 => sys_getpid(),
		60 => sys_exit(arg1 as i32),
		63 => sys_uname(arg1 as *mut u8),
		_ => {
			println!("Unimplemented system call: {}", syscall_num);
			Err(SyscallError::InvalidArgument)
		}
	}
}

/// Read system call - placeholder implementation
fn sys_read(fd: usize, buf: *mut u8, count: usize) -> SyscallResult {
	// For now, return 0 bytes read for stdin
	if fd == 0 {
		Ok(0)
	} else {
		Err(SyscallError::BadFileNumber)
	}
}

/// Write system call - basic implementation for stdout/stderr
fn sys_write(fd: usize, buf: *const u8, count: usize) -> SyscallResult {
	if fd == 1 || fd == 2 {
		// stdout or stderr
		let slice = unsafe { core::slice::from_raw_parts(buf, count) };
		if let Ok(s) = core::str::from_utf8(slice) {
			print!("{}", s);
			Ok(count)
		} else {
			Err(SyscallError::InvalidArgument)
		}
	} else {
		Err(SyscallError::BadFileNumber)
	}
}

/// Open system call - placeholder implementation
fn sys_open(pathname: *const u8, flags: usize, mode: usize) -> SyscallResult {
	// TODO: Implement file system and file opening
	Err(SyscallError::NoSuchFileOrDirectory)
}

/// Close system call - placeholder implementation
fn sys_close(fd: usize) -> SyscallResult {
	// TODO: Implement file descriptor management
	if fd > 2 {
		Ok(0)
	} else {
		Err(SyscallError::BadFileNumber)
	}
}

/// Get process ID system call
fn sys_getpid() -> SyscallResult {
	// Return process ID 1 for now (init process)
	Ok(1)
}

/// Exit the current process
fn sys_exit(status: i32) -> SyscallResult {
	println!("Process exiting with status: {}", status);
	// For now, just halt the system
	// In a real OS, this would terminate the current process
	hlt_loop();
}

/// Uname system call - return system information
fn sys_uname(buf: *mut u8) -> SyscallResult {
	let uname_info = b"ScottOS\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0v0.1.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0x86_64\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
	
	unsafe {
		core::ptr::copy_nonoverlapping(uname_info.as_ptr(), buf, uname_info.len().min(390));
	}
	
	Ok(0)
} 