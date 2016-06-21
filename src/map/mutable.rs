use std::mem;
use std::ptr;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

use libc::{c_void, size_t};

use nix::sys::mman::{mmap, munmap};
use nix::sys::mman::{PROT_READ, PROT_WRITE};
use nix::sys::mman::{MAP_SHARED};

use Safe;
use error::{self, Error};
use object::Object;

/// A mapping from a mutable shared object to a type.
pub struct Mutable<'a, T: Safe> {
	object:  &'a mut Object,
	address: *mut c_void,

	_marker: PhantomData<T>,
}

impl<'a, T: Safe> Mutable<'a, T> {
	/// Create a new mapping.
	pub fn new(object: &'a mut Object) -> error::Result<Mutable<'a, T>> {
		let size = mem::size_of::<T>() as isize;

		if size > object.size {
			return Err(Error::WrongSize);
		}

		let address = try!(mmap(ptr::null_mut(), size as size_t, PROT_READ | PROT_WRITE, MAP_SHARED, object.fd, 0));

		Ok(Mutable {
			object:  object,
			address: address,

			_marker: PhantomData,
		})
	}
}

impl<'a, T: Safe> Deref for Mutable<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe {
			(self.address as *const _ as *const _).as_ref().unwrap()
		}
	}
}

impl<'a, T: Safe> DerefMut for Mutable<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe {
			(self.address as *mut _).as_mut().unwrap()
		}
	}
}

impl<'a, T: Safe> Drop for Mutable<'a, T> {
	fn drop(&mut self) {
		munmap(self.address, self.object.size as size_t).unwrap();
	}
}
