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

/// Create a new shared object of the given type.
pub fn create<T: Safe, P: AsRef<Path>>(path: P) -> error::Result<Owned<T>> {
	Owned::new(try!(Object::create(path, mem::size_of::<T>() as isize)))
}

/// Open an existing shared object of the given type.
pub fn open<T: Safe, P: AsRef<Path>>(path: P) -> error::Result<Owned<T>> {
	Owned::new(try!(Object::open(path)))
}
