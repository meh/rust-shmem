extern crate shmem;

#[derive(Copy, Clone)]
pub struct Foo {
	bar: u32,
	baz: u32,
}

fn main() {
	let foo = shmem::open::<Foo, _>("shmem-rust-test").unwrap();

	println!("bar={} baz={}", foo.bar, foo.baz);
}
