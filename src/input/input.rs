use std::ops::Deref;
use std::thread;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use controller;
use super::{Event, State};

macro_rules! wait {
	($body:expr) => (
		if let Ok(value) = $body {
			value
		}
		else {
			thread::sleep(Duration::from_secs(1));
			continue;
		}
	);
}

macro_rules! end {
	($body:expr) => (
		if let Ok(value) = $body {
			value
		}
		else {
			break;
		}
	);
}

pub struct Input {
	rx: Receiver<Event>,
}

impl Input {
	pub fn spawn() -> Input {
		let (tx, rx) = channel();

		thread::spawn(move || {
			let mut manager = controller::Manager::new().unwrap();
			let mut state   = State::default();

			loop {
				let mut controller = wait!(manager.open());

				end!(tx.send(Event::Connected));

				loop {
					for event in state.update(end!(controller.state(Duration::from_secs(0)))) {
						end!(tx.send(event))
					}
				}

				end!(tx.send(Event::Disconnected));
			}
		});

		Input {
			rx: rx
		}
	}
}

impl Deref for Input {
	type Target = Receiver<Event>;

	fn deref(&self) -> &Self::Target {
		&self.rx
	}
}
