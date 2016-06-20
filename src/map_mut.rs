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

/// A mapping from a mutable shared object to a type.
pub struct MapMut<'a, T: Copy + 'static> {
	object:  &'a mut Object,
	address: *mut c_void,

	_marker: PhantomData<&'a mut T>,
}

impl<'a, T: Copy + 'static> MapMut<'a, T> {
	#[doc(hidden)]
	pub fn new(object: &'a mut Object) -> error::Result<MapMut<'a, T>> {
		let size = mem::size_of::<T>() as isize;

		if size > object.size {
			return Err(Error::WrongSize);
		}

		let address = try!(mmap(ptr::null_mut(), size as size_t, PROT_READ | PROT_WRITE, MAP_SHARED, object.fd, 0));

		Ok(MapMut {
			object:  object,
			address: address,

			_marker: PhantomData,
		})
	}
}

impl<'a, T: Copy + 'static> Deref for MapMut<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe {
			(self.address as *const _ as *const _).as_ref().unwrap()
		}
	}
}

impl<'a, T: Copy + 'static> DerefMut for MapMut<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe {
			(self.address as *mut _).as_mut().unwrap()
		}
	}
}

impl<'a, T: Copy + 'static> Drop for MapMut<'a, T> {
	fn drop(&mut self) {
		munmap(self.address, self.object.size as size_t).unwrap();
	}
}
