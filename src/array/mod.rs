use std::mem;
use std::path::Path;

use Safe;
use error;
use object::Object;

mod owned;
pub use self::owned::Owned;

mod reference;
pub use self::reference::Reference;

mod mutable;
pub use self::mutable::Mutable;

/// Create a new array of the given type and size as a shared object.
pub fn create<T: Safe, P: AsRef<Path>>(path: P, size: usize) -> error::Result<Owned<T>> {
	Owned::new(try!(Object::create(path, (mem::size_of::<T>() * size) as isize)))
}

/// Open an existing shared object as an array of the given type.
///
/// The size is calculated from the size of the shared object.
pub fn open<T: Safe, P: AsRef<Path>>(path: P) -> error::Result<Owned<T>> {
	Owned::new(try!(Object::open(path)))
}
