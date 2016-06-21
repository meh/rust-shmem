#[macro_use]
extern crate nix;
extern crate libc;

/// Marker to define a type as safe to share.
pub unsafe trait Safe { }
unsafe impl<T: Copy + 'static> Safe for T { }

mod error;
pub use error::{Error, Result};

mod object;
pub use object::Object;

pub mod array;
pub use array::{Owned as Array, Reference as ArrayRef, Mutable as ArrayMut};

pub mod map;
pub use map::{Owned as Map, Reference as MapRef, Mutable as MapMut};
pub use map::{create, open};
