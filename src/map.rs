use std::mem;
use std::ptr;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

use libc::{c_void, size_t};

use nix::sys::mman::{mmap, munmap};
use nix::sys::mman::{PROT_READ, PROT_WRITE};
use nix::sys::mman::{MAP_SHARED};

use error::{self, Error};
use object::Object;

/// A mapping from an owned shared object to a type.
pub struct Map<T: Copy + 'static> {
	object:  Object,
	address: *mut c_void,

	_marker: PhantomData<T>,
}

impl<T: Copy + 'static> Map<T> {
	#[doc(hidden)]
	pub fn new(object: Object) -> error::Result<Map<T>> {
		let size = mem::size_of::<T>() as isize;

		if size > object.size {
			return Err(Error::WrongSize);
		}

		let address = try!(mmap(ptr::null_mut(), size as size_t, PROT_READ | PROT_WRITE, MAP_SHARED, object.fd, 0));

		Ok(Map {
			object:  object,
			address: address,

			_marker: PhantomData,
		})
	}
}

impl<T: Copy + 'static> Deref for Map<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe {
			(self.address as *const _ as *const _).as_ref().unwrap()
		}
	}
}

impl<T: Copy + 'static> DerefMut for Map<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe {
			(self.address as *mut _).as_mut().unwrap()
		}
	}
}

impl<T: Copy + 'static> Drop for Map<T> {
	fn drop(&mut self) {
		munmap(self.address, self.object.size as size_t).unwrap();
	}
}
