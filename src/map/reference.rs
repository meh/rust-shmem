use std::mem;
use std::ptr;
use std::ops::Deref;
use std::marker::PhantomData;

use libc::{c_void, size_t};

use nix::sys::mman::{mmap, munmap};
use nix::sys::mman::{PROT_READ};
use nix::sys::mman::{MAP_SHARED};

use Safe;
use error::{self, Error};
use object::Object;

/// A mapping from a shared object to a type.
pub struct Reference<'a, T: Safe> {
	object:  &'a Object,
	address: *mut c_void,

	_marker: PhantomData<T>,
}

impl<'a, T: Safe> Reference<'a, T> {
	/// Create a new mapping.
	pub fn new(object: &'a Object) -> error::Result<Reference<'a, T>> {
		let size = mem::size_of::<T>() as isize;

		if size > object.size {
			return Err(Error::WrongSize);
		}

		let address = try!(mmap(ptr::null_mut(), object.size as size_t, PROT_READ, MAP_SHARED, object.fd, 0));

		Ok(Reference {
			object:  object,
			address: address,

			_marker: PhantomData,
		})
	}
}

impl<'a, T: Safe> Deref for Reference<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe {
			(self.address as *const _ as *const _).as_ref().unwrap()
		}
	}
}

impl<'a, T: Safe> Drop for Reference<'a, T> {
	fn drop(&mut self) {
		munmap(self.address, self.object.size as size_t).unwrap();
	}
}
