use std::ops::Deref;
use std::thread;
use std::sync::mpsc::{channel, Receiver};
use std::time::{Instant, Duration};
use controller;
use super::{Event, State};

pub struct Input {
	rx: Receiver<(Instant, Event)>,
}

impl Input {
	pub fn spawn() -> Input {
		let (tx, rx) = channel();

		thread::spawn(move || {
			let mut manager = controller::Manager::new().unwrap();
			let mut state   = State::default();

			loop {
				let mut controller = wait!(manager.open(), 500);

				end!(tx.send((Instant::now(), Event::Connected)));

				loop {
					let new = end!(controller.state(Duration::from_secs(0)));
					let at  = Instant::now();

					for event in state.update(new) {
						end!(tx.send((at, event)))
					}
				}

				end!(tx.send((Instant::now(), Event::Disconnected)));
			}
		});

		Input {
			rx: rx
		}
	}
}

impl Deref for Input {
	type Target = Receiver<(Instant, Event)>;

	fn deref(&self) -> &Self::Target {
		&self.rx
	}
}
