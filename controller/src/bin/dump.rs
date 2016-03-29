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
		let buffer = controller.raw(Duration::from_secs(0)).unwrap();

		for (n, byte) in (&buffer[..]).iter().enumerate() {
			print!("{:02x}", byte);

			if (n + 1) % 4 == 0 {
				print!(" ");
			}
		}

		println!("");
	}
}
