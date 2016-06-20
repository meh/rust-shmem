use std::mem;
use std::ptr;
use std::ops::Deref;
use std::marker::PhantomData;

use libc::{c_void, size_t};

use nix::sys::mman::{mmap, munmap};
use nix::sys::mman::{PROT_READ};
use nix::sys::mman::{MAP_SHARED};

use error::{self, Error};
use object::Object;

/// A mapping from a shared object to a type.
pub struct MapRef<'a, T: Copy + 'static> {
	object:  &'a Object,
	address: *mut c_void,

	_marker: PhantomData<&'a T>,
}

impl<'a, T: Copy + 'static> MapRef<'a, T> {
	#[doc(hidden)]
	pub fn new(object: &'a Object) -> error::Result<MapRef<'a, T>> {
		let size = mem::size_of::<T>() as isize;

		if size > object.size {
			return Err(Error::WrongSize);
		}

		let address = try!(mmap(ptr::null_mut(), size as size_t, PROT_READ, MAP_SHARED, object.fd, 0));

		Ok(MapRef {
			object:  object,
			address: address,

			_marker: PhantomData,
		})
	}
}

impl<'a, T: Copy + 'static> Deref for MapRef<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe {
			(self.address as *const _ as *const _).as_ref().unwrap()
		}
	}
}

impl<'a, T: Copy + 'static> Drop for MapRef<'a, T> {
	fn drop(&mut self) {
		munmap(self.address, self.object.size as size_t).unwrap();
	}
}
