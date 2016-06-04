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

	let power = Definition::default()
		.field(Field::named("event")
			.bytes(1));

	let idle = Definition::default()
		.field(Field::named("sequence")
			.is::<u32>(LittleEndian)
			.style(Color::Fixed(255).normal()))
		.field(Field::padding()
			.bytes(4))
		.field(Field::unknown()
			.bytes(4));

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
		.field(Field::named("trigger.left.precise")
			.is::<u16>(LittleEndian)
			.style(Color::Fixed(255).on(Color::Fixed(63)).underline()))
		.field(Field::named("trigger.right.precise")
			.is::<u16>(LittleEndian)
			.style(Color::Fixed(255).on(Color::Fixed(63))))
		.field(Field::padding()
			.bytes(8))
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

	let fmt = if matches.is_present("structured") {
		let mut fmt = formatter::Structured::default()
			.header(true);

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
		let (id, buffer) = controller.receive(Duration::from_secs(0)).unwrap();

		match id {
			0x03 =>
				fmt.format(&power, buffer, io::stdout()).unwrap(),

			0x04 =>
				fmt.format(&idle, buffer, io::stdout()).unwrap(),

			0x01 =>
				fmt.format(&input, buffer, io::stdout()).unwrap(),

			_ => ()
		}

		println!("");
	}
}
