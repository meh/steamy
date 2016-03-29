extern crate steamy_vdf as vdf;
use std::fs::File;

#[test]
fn file() {
	let mut reader = vdf::Reader::from(File::open("tests/desktop.vdf").unwrap());

	println!("{:?}", reader.event());
}
