use std::path::{Path, PathBuf};
use std::os::unix::io::RawFd;

use libc::off_t;

use nix::sys::mman::{shm_open, shm_unlink};
use nix::unistd::{ftruncate, close};
use nix::sys::stat::{fstat};
use nix::fcntl::{O_CREAT, O_RDWR};
use nix::sys::stat::{S_IRUSR, S_IWUSR};

use error;

/// A shared object.
pub struct Object {
	pub path: PathBuf,
	pub fd:   RawFd,
	pub size: isize,

	created: bool,
}

impl Object {
	/// Create a shared object.
	pub fn create<P: AsRef<Path>>(path: P, size: isize) -> error::Result<Object> {
		let fd = try!(shm_open(path.as_ref(), O_CREAT | O_RDWR, S_IRUSR | S_IWUSR));
		try!(ftruncate(fd, size as off_t));

		Ok(Object {
			path: path.as_ref().to_path_buf(),
			fd:   fd,
			size: size,

			created: true,
		})
	}

	/// Open an existing shared object.
	pub fn open<P: AsRef<Path>>(path: P) -> error::Result<Object> {
		let fd = try!(shm_open(path.as_ref(), O_RDWR, S_IRUSR | S_IWUSR));
		let stat = try!(fstat(fd));

		Ok(Object {
			path: path.as_ref().to_path_buf(),
			fd:   fd,
			size: stat.st_size as isize,

			created: false,
		})
	}
}

impl Drop for Object {
	fn drop(&mut self) {
		close(self.fd).unwrap();
		
		if self.created {
			shm_unlink(&self.path).unwrap();
		}
	}
}
