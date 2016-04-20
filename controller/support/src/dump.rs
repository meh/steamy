extern crate clap;
use clap::{Arg, App};

extern crate steamy_controller as controller;

extern crate rorschach;
use rorschach::{Definition, Field, Formatter, formatter, LittleEndian};
use rorschach::formatter::Color;

use std::io::{self, Read, Write};
use std::time::Duration;

enum Format {
	Structured(formatter::Structured),
	Inline(formatter::Inline),
}

impl Formatter for Format {
	fn format<R: Read, W: Write>(&self, def: &Definition, input: R, output: W) -> io::Result<()> {
		match self {
			&Format::Structured(ref fmt) =>
				fmt.format(def, input, output),

			&Format::Inline(ref fmt) =>
				fmt.format(def, input, output)
		}
	}
}

fn main() {
	let matches = App::new("dump")
		.version("1.0")
		.author("meh <meh@schizofreni.co>")
		.about("Dump the raw packets from the controller.")
			.arg(Arg::with_name("sensors")
				.short("S")
				.long("sensors")
				.help("Enable the gyroscope and accelerometer."))
			.arg(Arg::with_name("structured")
				.short("s")
				.long("structured")
				.help("Enable structured output."))
			.arg(Arg::with_name("colored")
				.short("c")
				.long("color")
				.help("Enable colored output."))
		.get_matches();

	let mut manager    = controller::Manager::new().unwrap();
	let mut controller = manager.open().unwrap();

	if matches.is_present("sensors") {
		controller.sensors().on().unwrap();
	}

	let header = Definition::default()
		.field(Field::constant()
			.bytes(1)
			.value(0x01))
		.field(Field::padding()
			.bytes(1))
		.field(Field::named("status")
			.bytes(2)
			.style(Color::Black.on(Color::Fixed(255))));

	let power = Definition::default()
		.field(Field::named("event")
			.bytes(1))
		.field(Field::unknown()
			.bytes(3))
		.field(Field::padding()
			.bytes(56));

	let idle = Definition::default()
		.field(Field::named("sequence")
			.is::<u32>(LittleEndian)
			.style(Color::Fixed(255).normal()))
		.field(Field::padding()
			.bytes(4))
		.field(Field::unknown()
			.bytes(4))
		.field(Field::padding()
			.bytes(48));

	let input = Definition::default()
		.field(Field::named("sequence")
			.is::<u32>(LittleEndian)
			.style(Color::Fixed(255).normal()))
		.field(Field::named("buttons")
			.bytes(3)
			.binary()
			.style(Color::Fixed(3).normal()))
		.field(Field::named("trigger.left")
			.is::<u8>(LittleEndian)
			.style(Color::Fixed(255).on(Color::Fixed(63)).underline()))
		.field(Field::named("trigger.right")
			.is::<u8>(LittleEndian)
			.style(Color::Fixed(255).on(Color::Fixed(63))))
		.field(Field::padding()
			.bytes(3))
		.field(Field::named("pad.left.x")
			.is::<i16>(LittleEndian)
			.style(Color::Fixed(255).on(Color::Fixed(27)).underline()))
		.field(Field::named("pad.left.y")
			.is::<i16>(LittleEndian)
			.style(Color::Fixed(27).normal()))
		.field(Field::named("pad.right.x")
			.is::<i16>(LittleEndian)
			.style(Color::Fixed(255).on(Color::Fixed(36)).underline()))
		.field(Field::named("pad.right.y")
			.is::<i16>(LittleEndian)
			.style(Color::Fixed(36).normal()))
		.field(Field::padding()
			.bytes(12))
		.field(Field::named("acceleration.pitch")
			.is::<i16>(LittleEndian)
			.style(Color::Fixed(124).normal()))
		.field(Field::named("acceleration.yaw")
			.is::<i16>(LittleEndian)
			.style(Color::Fixed(160).normal()))
		.field(Field::named("acceleration.roll")
			.is::<i16>(LittleEndian)
			.style(Color::Fixed(196).normal()))
		.field(Field::named("orientation.pitch")
			.is::<i16>(LittleEndian)
			.style(Color::Fixed(57).normal()))
		.field(Field::named("orientation.yaw")
			.is::<i16>(LittleEndian)
			.style(Color::Fixed(93).normal()))
		.field(Field::named("orientation.roll")
			.is::<i16>(LittleEndian)
			.style(Color::Fixed(129).normal()))
		.field(Field::padding()
			.bytes(16));

	let unknown = Definition::default()
		.field(Field::unknown()
			.bytes(60));

	let fmt = if matches.is_present("structured") {
		let mut fmt = formatter::Structured::default()
			.header(false);

		if matches.is_present("colored") {
			fmt = fmt.style(Default::default());
		}

		Format::Structured(fmt)
	}
	else {
		let mut fmt = formatter::Inline::default()
			.split(4);

		if matches.is_present("colored") {
			fmt = fmt.style(Default::default());
		}

		Format::Inline(fmt)
	};

	loop {
		let buffer = controller.raw(Duration::from_secs(0)).unwrap();

		match fmt {
			Format::Structured(ref fmt) =>
				fmt.clone().header(true).format(&header, &buffer[..4], io::stdout()).unwrap(),

			Format::Inline(ref fmt) =>
				fmt.clone().newline(false).format(&header, &buffer[..4], io::stdout()).unwrap()
		}

		match (buffer[2], buffer[3]) {
			(0x03, 0x01) =>
				fmt.format(&power, &buffer[4..], io::stdout()).unwrap(),

			(0x04, 0x0b) =>
				fmt.format(&idle, &buffer[4..], io::stdout()).unwrap(),

			(0x01, 0x3c) =>
				fmt.format(&input, &buffer[4..], io::stdout()).unwrap(),

			_ =>
				fmt.format(&unknown, &buffer[4..], io::stdout()).unwrap(),
		}

		println!("");
	}
}
