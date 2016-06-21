use std::mem;
use std::ptr;
use std::slice;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

use libc::{c_void, size_t};

use nix::sys::mman::{mmap, munmap};
use nix::sys::mman::{PROT_READ, PROT_WRITE};
use nix::sys::mman::{MAP_SHARED};

use Safe;
use error::{self, Error};
use object::Object;

/// A mapping from an owned shared object to a slice of a type.
pub struct Owned<T: Safe> {
	object:  Object,
	address: *mut c_void,

	_marker: PhantomData<T>,
}

impl<T: Safe> Owned<T> {
	/// Create a new type mapping.
	pub fn new(object: Object) -> error::Result<Owned<T>> {
		let size = mem::size_of::<T>() as isize;

		if object.size % size != 0 {
			return Err(Error::WrongSize);
		}

		let address = try!(mmap(ptr::null_mut(), object.size as size_t, PROT_READ | PROT_WRITE, MAP_SHARED, object.fd, 0));

		Ok(Owned {
			object:  object,
			address: address,

			_marker: PhantomData,
		})
	}
}

impl<T: Safe> Deref for Owned<T> {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		unsafe {
			slice::from_raw_parts(self.address as *const _ as *const _, self.object.size as usize / mem::size_of::<T>())
		}
	}
}

impl<T: Safe> DerefMut for Owned<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe {
			slice::from_raw_parts_mut(self.address as *mut _, self.object.size as usize / mem::size_of::<T>())
		}
	}
}

impl<T: Safe> Drop for Owned<T> {
	fn drop(&mut self) {
		munmap(self.address, self.object.size as size_t).unwrap();
	}
}
