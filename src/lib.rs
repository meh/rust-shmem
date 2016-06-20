#[macro_use]
extern crate nix;
extern crate libc;

use std::path::Path;
use std::mem;

mod error;
pub use error::{Error, Result};

mod object;
pub use object::Object;

mod map;
pub use map::Map;

mod map_ref;
pub use map_ref::MapRef;

mod map_mut;
pub use map_mut::MapMut;

pub fn create<T: Copy + 'static, P: AsRef<Path>>(path: P) -> Result<Map<T>> {
	try!(Object::create(path, mem::size_of::<T>() as isize)).into()
}

pub fn open<T: Copy + 'static, P: AsRef<Path>>(path: P) -> Result<Map<T>> {
	try!(Object::open(path)).into()
}
