extern crate shmem;

use std::thread;
use std::time::Duration;

#[derive(Copy, Clone)]
pub struct Foo {
	bar: u32,
	baz: u32,
}

fn main() {
	let mut foo = shmem::create::<Foo, _>("shmem-rust-test").unwrap();

	foo.bar = 23;
	foo.baz = 42;

	thread::sleep(Duration::from_secs(5));
}
