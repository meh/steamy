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
		0x83 => {
			Definition::default()
				.field(Field::constant()
					.bytes(1)
					.value(0x83))
				.field(Field::constant()
					.bytes(1)
					.value(0x23))
				.field(Field::named("key")
					.bytes(1)
					.style(Color::Fixed(1).underline()))
				.field(Field::named("value")
					.bytes(4)
					.style(Color::Fixed(1).normal()))
				.field(Field::named("key")
					.bytes(1)
					.style(Color::Fixed(2).underline()))
				.field(Field::named("value")
					.bytes(4)
					.style(Color::Fixed(2).normal()))
				.field(Field::named("key")
					.bytes(1)
					.style(Color::Fixed(3).underline()))
				.field(Field::named("value")
					.bytes(4)
					.style(Color::Fixed(3).normal()))
				.field(Field::named("key")
					.bytes(1)
					.style(Color::Fixed(4).underline()))
				.field(Field::named("value")
					.bytes(4)
					.style(Color::Fixed(4).normal()))
				.field(Field::named("key.firmware")
					.bytes(1)
					.style(Color::Fixed(5).underline()))
				.field(Field::named("value.firmware")
					.is::<i32>(LittleEndian)
					.style(Color::Fixed(5).normal()))
				.field(Field::named("key")
					.bytes(1)
					.style(Color::Fixed(6).underline()))
				.field(Field::named("value")
					.bytes(4)
					.style(Color::Fixed(6).normal()))
				.field(Field::named("key")
					.bytes(1)
					.style(Color::Fixed(7).underline()))
				.field(Field::named("value")
					.bytes(4)
					.style(Color::Fixed(7).normal()))
		}

		0xba => {
			Definition::default()
				.field(Field::unknown()
					.bytes(64))
		}

		0xab => {
			Definition::default()
				.field(Field::unknown()
					.bytes(64))
		}

		_ => {
			Definition::default()
				.field(Field::unknown()
					.bytes(64))
		}
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
