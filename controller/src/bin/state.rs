extern crate steamy_controller as controller;
use controller::button;

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

				if buttons.bits() != 0 {
					println!("\tbuttons: {:?}", buttons);
				}

				if trigger.left != 0.0 || trigger.right != 0.0 {
					println!("\ttrigger: {:?}", trigger);
				}

				if buttons.contains(button::PAD_TOUCH) || buttons.contains(button::TRACK_TOUCH) {
					println!("\tpad: {:?}", pad);
				}

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
