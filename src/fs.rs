use alloc::{collections::BTreeMap, string::String, vec::Vec, format};
use alloc::string::ToString;
use spin::Mutex;

/// File system error types
#[derive(Debug, Clone, Copy)]
pub enum FsError {
	NotFound,
	PermissionDenied,
	AlreadyExists,
	IsDirectory,
	NotDirectory,
	InvalidPath,
	IoError,
}

/// File types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
	Regular,
	Directory,
	Symlink,
	Device,
}

/// File metadata
#[derive(Debug, Clone)]
pub struct FileMetadata {
	pub file_type: FileType,
	pub size: usize,
	pub permissions: u32,
	pub created: u64,
	pub modified: u64,
	pub accessed: u64,
}

/// In-memory file representation
#[derive(Debug, Clone)]
pub struct File {
	pub metadata: FileMetadata,
	pub data: Vec<u8>,
}

/// File descriptor
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileDescriptor(pub usize);

/// File handle with position tracking
#[derive(Debug)]
pub struct FileHandle {
	pub file: File,
	pub position: usize,
	pub flags: u32,
}

/// Simple in-memory file system
pub struct FileSystem {
	files: BTreeMap<String, File>,
	open_files: BTreeMap<FileDescriptor, FileHandle>,
	next_fd: usize,
}

impl FileSystem {
	/// Create a new file system
	pub fn new() -> Self {
		let mut fs = FileSystem {
			files: BTreeMap::new(),
			open_files: BTreeMap::new(),
			next_fd: 3, // Start after stdin, stdout, stderr
		};

		// Create root directory
		fs.create_directory("/".to_string()).unwrap();
		
		// Create basic directories
		fs.create_directory("/bin".to_string()).unwrap();
		fs.create_directory("/etc".to_string()).unwrap();
		fs.create_directory("/home".to_string()).unwrap();
		fs.create_directory("/tmp".to_string()).unwrap();
		fs.create_directory("/usr".to_string()).unwrap();
		fs.create_directory("/var".to_string()).unwrap();

		// Create /etc/passwd file
		let passwd_content = b"root:x:0:0:root:/root:/bin/sh\n";
		fs.create_file("/etc/passwd".to_string(), passwd_content.to_vec()).unwrap();

		fs
	}

	/// Create a new file
	pub fn create_file(&mut self, path: String, data: Vec<u8>) -> Result<(), FsError> {
		if self.files.contains_key(&path) {
			return Err(FsError::AlreadyExists);
		}

		let file = File {
			metadata: FileMetadata {
				file_type: FileType::Regular,
				size: data.len(),
				permissions: 0o644,
				created: 0, // TODO: Add real timestamp
				modified: 0,
				accessed: 0,
			},
			data,
		};

		self.files.insert(path, file);
		Ok(())
	}

	/// Create a new directory
	pub fn create_directory(&mut self, path: String) -> Result<(), FsError> {
		if self.files.contains_key(&path) {
			return Err(FsError::AlreadyExists);
		}

		let file = File {
			metadata: FileMetadata {
				file_type: FileType::Directory,
				size: 0,
				permissions: 0o755,
				created: 0,
				modified: 0,
				accessed: 0,
			},
			data: Vec::new(),
		};

		self.files.insert(path, file);
		Ok(())
	}

	/// Open a file
	pub fn open(&mut self, path: &str, flags: u32) -> Result<FileDescriptor, FsError> {
		let file = self.files.get(path).ok_or(FsError::NotFound)?.clone();

		let fd = FileDescriptor(self.next_fd);
		self.next_fd += 1;

		let handle = FileHandle {
			file,
			position: 0,
			flags,
		};

		self.open_files.insert(fd, handle);
		Ok(fd)
	}

	/// Close a file
	pub fn close(&mut self, fd: FileDescriptor) -> Result<(), FsError> {
		self.open_files.remove(&fd).ok_or(FsError::NotFound)?;
		Ok(())
	}

	/// Read from a file
	pub fn read(&mut self, fd: FileDescriptor, buffer: &mut [u8]) -> Result<usize, FsError> {
		let handle = self.open_files.get_mut(&fd).ok_or(FsError::NotFound)?;
		
		let available = handle.file.data.len().saturating_sub(handle.position);
		let to_read = buffer.len().min(available);
		
		if to_read > 0 {
			buffer[..to_read].copy_from_slice(
				&handle.file.data[handle.position..handle.position + to_read]
			);
			handle.position += to_read;
		}
		
		Ok(to_read)
	}

	/// Write to a file
	pub fn write(&mut self, fd: FileDescriptor, buffer: &[u8]) -> Result<usize, FsError> {
		let handle = self.open_files.get_mut(&fd).ok_or(FsError::NotFound)?;
		
		// For simplicity, append to the end of the file
		handle.file.data.extend_from_slice(buffer);
		handle.file.metadata.size = handle.file.data.len();
		
		Ok(buffer.len())
	}

	/// Get file metadata
	pub fn stat(&self, path: &str) -> Result<FileMetadata, FsError> {
		let file = self.files.get(path).ok_or(FsError::NotFound)?;
		Ok(file.metadata.clone())
	}

	/// List directory contents
	pub fn list_directory(&self, path: &str) -> Result<Vec<String>, FsError> {
		let file = self.files.get(path).ok_or(FsError::NotFound)?;
		
		if file.metadata.file_type != FileType::Directory {
			return Err(FsError::NotDirectory);
		}

		// Return all files that start with the directory path
		let mut entries = Vec::new();
		let prefix = if path.ends_with('/') {
			path.to_string()
		} else {
			format!("{}/", path)
		};

		for (file_path, _) in &self.files {
			if file_path.starts_with(&prefix) && file_path != path {
				let relative_path = &file_path[prefix.len()..];
				if !relative_path.contains('/') && !relative_path.is_empty() {
					entries.push(relative_path.to_string());
				}
			}
		}

		Ok(entries)
	}
}

/// Global file system instance
static FILE_SYSTEM: Mutex<FileSystem> = Mutex::new(FileSystem { 
	files: BTreeMap::new(),
	open_files: BTreeMap::new(),
	next_fd: 3,
});

/// Initialize the file system with default directories and files
pub fn init_filesystem() {
	with_filesystem(|fs| {
		// Create standard UNIX directories
		fs.create_directory("/".to_string()).unwrap();
		
		// Create standard directories
		fs.create_directory("/bin".to_string()).unwrap();
		fs.create_directory("/etc".to_string()).unwrap();
		fs.create_directory("/home".to_string()).unwrap();
		fs.create_directory("/tmp".to_string()).unwrap();
		fs.create_directory("/usr".to_string()).unwrap();
		fs.create_directory("/var".to_string()).unwrap();
		
		// Create a basic passwd file
		let passwd_content = b"root:x:0:0:root:/root:/bin/sh\n";
		fs.create_file("/etc/passwd".to_string(), passwd_content.to_vec()).unwrap();
	});
}

/// Execute a function with access to the global file system
pub fn with_filesystem<F, R>(f: F) -> R
where
	F: FnOnce(&mut FileSystem) -> R,
{
	f(&mut FILE_SYSTEM.lock())
} 