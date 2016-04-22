extern crate clap;
use clap::{Arg, App};

extern crate steamy_controller as controller;

extern crate rorschach;
use rorschach::{Definition, Field, Formatter, formatter, LittleEndian};
use rorschach::formatter::Color;

use std::u8;
use std::io;

fn main() {
	let matches = App::new("dump")
		.version("1.0")
		.author("meh <meh@schizofreni.co>")
		.about("Dump the raw response from the controller.")
			.arg(Arg::with_name("structured")
				.short("s")
				.long("structured")
				.help("Enable structured output."))
			.arg(Arg::with_name("colored")
				.short("c")
				.long("color")
				.help("Enable colored output."))
			.arg(Arg::with_name("ID")
				.required(true)
				.index(1)
				.help("The request ID."))
		.get_matches();

	let mut manager    = controller::Manager::new().unwrap();
	let mut controller = manager.open().unwrap();

	let id  = u8::from_str_radix(matches.value_of("ID").unwrap(), 16).unwrap();
	let buf = controller.request(id, 255).unwrap();

	let def = match id {
		0x83 =>
			Definition::default()
				.field(Field::unknown()
					.bytes(23))
				.field(Field::named("firmware")
					.is::<i32>(LittleEndian)
					.style(Color::Fixed(255).normal()))
				.field(Field::unknown()
					.bytes(37)),

		_ =>
			Definition::default()
				.field(Field::unknown()
					.bytes(64))
	};

	if matches.is_present("structured") {
		let mut fmt = formatter::Structured::default()
			.header(true);

		if matches.is_present("colored") {
			fmt = fmt.style(Default::default());
		}

		fmt.format(&def, &buf[..], io::stdout()).unwrap();
	}
	else {
		let mut fmt = formatter::Inline::default()
			.newline(true)
			.split(4);

		if matches.is_present("colored") {
			fmt = fmt.style(Default::default());
		}

		fmt.format(&def, &buf[..], io::stdout()).unwrap();
	}
}
