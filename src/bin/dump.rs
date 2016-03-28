extern crate steamy_controller as controller;
use std::time::Duration;

use std::env;

fn main() {
	let mut manager    = controller::Manager::new().unwrap();
	let mut controller = manager.open().unwrap();
	let sensors        = env::var("SENSORS").unwrap_or(String::from("off")) == "on";

	if sensors {
		controller.sensors().on().unwrap();
	}

	loop {
		match controller.state(Duration::from_secs(0)).unwrap() {
			controller::State::Input { sequence, buttons, trigger, pad, orientation, acceleration, .. } => {
				println!("{} {{", sequence);
				println!("\tbuttons: {:?}", buttons);
				println!("\ttrigger: {:?}", trigger);
				println!("\tpad: {:?}", pad);

				if env::var("SENSORS").is_ok() {
					println!("\torientation: {:?}", orientation);
					println!("\tacceleration: {:?}", acceleration);
				}

				println!("}}");
				println!("");
			}

			controller::State::Power(state) => {
				if state {
					println!("-- ON --");

					if sensors {
						controller.sensors().on().unwrap();
					}
				}
				else {
					println!("-- OFF --");
				}
			}

			_ => ()
		}
	}
}
