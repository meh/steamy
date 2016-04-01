extern crate steamy_controller as controller;
use controller::button;
use controller::sound::Note;

use std::time::Duration;
use std::thread;

fn main() {
	let mut manager    = controller::Manager::new().unwrap();
	let mut controller = manager.open().unwrap();
	let mut mode       = 0;

	loop {
		match controller.state(Duration::from_secs(0)).unwrap() {
			controller::State::Input { buttons, trigger, pad, .. } => {
				if buttons.contains(button::HOME) {
					mode += 1;
				}

				if buttons.intersects(button::A | button::B | button::X | button::Y) {
					let mut builder = controller.sound().left();

					if buttons.contains(button::RIGHT_GRIP | button::LEFT_GRIP) {
						if buttons.contains(button::A) {
							builder = builder.note(Note::D).octave(7);
						}
						else if buttons.contains(button::X) {
							builder = builder.note(Note::E).octave(7);
						}
						else if buttons.contains(button::Y) {
							builder = builder.note(Note::F).octave(7);
						}
						else if buttons.contains(button::B) {
							builder = builder.note(Note::G).octave(7);
						}
					}
					else if buttons.contains(button::RIGHT_GRIP) {
						if buttons.contains(button::A) {
							builder = builder.note(Note::G).octave(6);
						}
						else if buttons.contains(button::X) {
							builder = builder.note(Note::A).octave(6);
						}
						else if buttons.contains(button::Y) {
							builder = builder.note(Note::B).octave(6);
						}
						else if buttons.contains(button::B) {
							builder = builder.note(Note::C).octave(7);
						}
					}
					else if buttons.contains(button::LEFT_GRIP) {
						if buttons.contains(button::A) {
							builder = builder.note(Note::F).octave(5);
						}
						else if buttons.contains(button::X) {
							builder = builder.note(Note::G).octave(5);
						}
						else if buttons.contains(button::Y) {
							builder = builder.note(Note::A).octave(5);
						}
						else if buttons.contains(button::B) {
							builder = builder.note(Note::B).octave(5);
						}
					}
					else {
						if buttons.contains(button::A) {
							builder = builder.note(Note::C).octave(6);
						}
						else if buttons.contains(button::X) {
							builder = builder.note(Note::D).octave(6);
						}
						else if buttons.contains(button::Y) {
							builder = builder.note(Note::E).octave(6);
						}
						else if buttons.contains(button::B) {
							builder = builder.note(Note::F).octave(6);
						}
					}

					if buttons.contains(button::BACK) {
						builder = builder.sharp();
					}

					builder.play().unwrap();
				}
				else {
					controller.sound().left().stop();
				}
			}

			_ => ()
		}
	}
}
