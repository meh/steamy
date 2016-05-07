#![feature(question_mark, slice_patterns, pub_restricted)]

extern crate steamy_controller as controller;
extern crate steamy_vdf as vdf;

extern crate clap;
use clap::{Arg, App};

#[cfg(target_os = "linux")]
extern crate uinput;

#[macro_use]
mod util;

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

	let     config = config::load(matches.value_of("CONFIG").unwrap()).expect("config: failed to load");
	let     input  = input::spawn();
	let mut mapper = mapper::new(&config).expect("mapper: failed to create");

	println!("{:#?}", config);

	for (at, event) in input.iter() {
		end!(mapper.event(at, event));
	}
}
