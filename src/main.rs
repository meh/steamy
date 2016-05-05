#![feature(question_mark, slice_patterns, pub_restricted)]

extern crate steamy_controller as controller;
extern crate steamy_vdf as vdf;

extern crate clap;
use clap::{Arg, App};

#[cfg(target_os = "linux")]
extern crate uinput;

mod error;
pub use error::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

mod config;
pub use config::Config;

mod input;
pub use input::Input;

mod mapper;
pub use mapper::Mapper;

fn main() {
	let matches = App::new("steamy")
		.version("1.0")
		.arg(Arg::with_name("CONFIG")
			.required(true)
			.index(1)
			.help("Path to the config file."))
		.get_matches();

	let mut mapper = mapper::new(config::load(matches.value_of("CONFIG").unwrap()).unwrap()).unwrap();
	let     input  = input::spawn();

	for event in input.iter() {
		mapper.send(event).unwrap();
	}
}
