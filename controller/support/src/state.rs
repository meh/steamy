extern crate clap;
use clap::{Arg, App};

extern crate steamy_controller as controller;

use std::time::Duration;

fn main() {
	let matches = App::new("state")
		.version("1.0")
		.author("meh <meh@schizofreni.co>")
		.about("Dump the controller state.")
			.arg(Arg::with_name("sensors")
				.short("S")
				.long("sensors")
				.help("Enable the gyroscope and accelerometer."))
		.get_matches();

	let mut manager    = controller::Manager::new().unwrap();
	let mut controller = manager.open().unwrap();

	if matches.is_present("sensors") {
		controller.sensors().on().unwrap();
	}

	loop {
		match controller.state(Duration::from_secs(0)).unwrap() {
			controller::State::Input { sequence, buttons, trigger, pad, orientation, acceleration, .. } => {
				println!("{} {{", sequence);

				if !buttons.is_empty() {
					println!("\tbuttons: {:?}", buttons);
				}

				if trigger.left != 0.0 || trigger.right != 0.0 {
					println!("\ttrigger: {:?}", trigger);
				}

				if !pad.right.is_empty() || !pad.left.is_empty() {
					println!("\tpad: {:?}", pad);
				}

				if matches.is_present("sensors") {
					println!("\torientation: {:?}", orientation);
					println!("\tacceleration: {:?}", acceleration);
				}

				println!("}}");
				println!("");
			}

			_ => ()
		}
	}
}
