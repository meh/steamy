extern crate steamy_controller as controller;
use controller::button;
use controller::sound::Note;

use std::time::Duration;
use std::ops::Deref;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Button {
	A,
	B,
	C,
	D,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct State {
	pub button:  Option<Button>,
	pub trigger: f32,
	pub grip:    bool,
	pub bumper:  bool,
	pub octave:  u8,
}

impl Default for State {
	fn default() -> Self {
		State {
			button:  None,
			trigger: 0.0,
			grip:    false,
			bumper:  false,
			octave:  6,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
struct Right {
	previous: State,
	current:  State,
}

impl Right {
	pub fn update(&mut self, buttons: controller::Button, trigger: controller::Trigger) {
		self.previous = self.current;

		if buttons.contains(button::A) {
			self.current.button = Some(Button::A);
		}
		else if buttons.contains(button::X) {
			self.current.button = Some(Button::B);
		}
		else if buttons.contains(button::Y) {
			self.current.button = Some(Button::C);
		}
		else if buttons.contains(button::B) {
			self.current.button = Some(Button::D);
		}
		else {
			self.current.button = None;
		}

		if buttons.contains(button::RIGHT_GRIP) {
			self.current.grip = true;
		}
		else {
			self.current.grip = false;
		}

		if buttons.contains(button::RIGHT_BUMPER) {
			self.current.bumper = true;
		}
		else {
			self.current.bumper = false;
		}

		if buttons.contains(button::FORWARD) {
			if buttons.contains(button::RIGHT_GRIP) {
				self.current.octave -= 1;
			}
			else {
				self.current.octave += 1;
			}
		}

		self.current.trigger = trigger.right;
	}

	pub fn has_update(&self) -> bool {
		self.previous != self.current
	}
}

impl Deref for Right {
	type Target = State;

	fn deref(&self) -> &Self::Target {
		&self.current
	}
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
struct Left {
	previous: State,
	current:  State,
}

impl Left {
	pub fn update(&mut self, buttons: controller::Button, trigger: controller::Trigger, pad: controller::Pad) {
		self.previous = self.current;

		if !buttons.contains(button::PAD_TOUCH) && (pad.left.x != 0 || pad.left.y != 0) {
			let x = pad.left.x;
			let y = pad.left.y;

			if y < -15_000 && x > -15_000 && x < 15_000 {
				self.current.button = Some(Button::A);
			}
			else if x < -15_000 && y > -15_000 && y < 15_000 {
				self.current.button = Some(Button::B);
			}
			else if y > 15_000 && x > -15_000 && x < 15_000 {
				self.current.button = Some(Button::C);
			}
			else if x > 15_000 && y > -15_000 && y < 15_000 {
				self.current.button = Some(Button::D);
			}
			else {
				self.current.button = None;
			}
		}
		else {
			self.current.button = None;
		}

		if buttons.contains(button::LEFT_GRIP) {
			self.current.grip = true;
		}
		else {
			self.current.grip = false;
		}

		if buttons.contains(button::LEFT_BUMPER) {
			self.current.bumper = true;
		}
		else {
			self.current.bumper = false;
		}

		if buttons.contains(button::BACK) {
			if buttons.contains(button::LEFT_GRIP) {
				self.current.octave -= 1;
			}
			else {
				self.current.octave += 1;
			}
		}

		self.current.trigger = trigger.left;
	}

	pub fn has_update(&self) -> bool {
		self.previous != self.current
	}
}

impl Deref for Left {
	type Target = State;

	fn deref(&self) -> &Self::Target {
		&self.current
	}
}

fn build<'a, 'b>(mut builder: controller::Sound<'a, 'b>, state: &State) -> controller::Sound<'a, 'b> {
	let button = state.button.unwrap();

	if state.grip && state.trigger == 1.0 {
		builder = match button {
			Button::A => builder.note(Note::D).octave(state.octave + 1),
			Button::B => builder.note(Note::E).octave(state.octave + 1),
			Button::C => builder.note(Note::F).octave(state.octave + 1),
			Button::D => builder.note(Note::G).octave(state.octave + 1),
		}
	}
	else if state.grip {
		builder = match button {
			Button::A => builder.note(Note::F).octave(state.octave - 1),
			Button::B => builder.note(Note::G).octave(state.octave - 1),
			Button::C => builder.note(Note::A).octave(state.octave - 1),
			Button::D => builder.note(Note::B).octave(state.octave - 1),
		}
	}
	else if state.trigger == 1.0 {
		builder = match button {
			Button::A => builder.note(Note::G).octave(state.octave),
			Button::B => builder.note(Note::A).octave(state.octave),
			Button::C => builder.note(Note::B).octave(state.octave),
			Button::D => builder.note(Note::C).octave(state.octave + 1),
		}
	}
	else {
		builder = match button {
			Button::A => builder.note(Note::C).octave(state.octave),
			Button::B => builder.note(Note::D).octave(state.octave),
			Button::C => builder.note(Note::E).octave(state.octave),
			Button::D => builder.note(Note::F).octave(state.octave),
		}
	};

	if state.bumper {
		builder = builder.sharp();
	}

	builder
}

fn main() {
	let mut manager    = controller::Manager::new().unwrap();
	let mut controller = manager.open().unwrap();

	let mut left  = Left::default();
	let mut right = Right::default();

	loop {
		match controller.state(Duration::from_secs(0)).unwrap() {
			controller::State::Input { buttons, trigger, pad, .. } => {
				left.update(buttons, trigger, pad);
				right.update(buttons, trigger);

				if left.has_update() {
					if left.button.is_some() {
						build(controller.sound().left(), &left).play().unwrap();
					}
					else {
						controller.sound().left().stop().unwrap();
					}
				}

				if right.has_update() {
					if right.button.is_some() {
						build(controller.sound().right(), &right).play().unwrap();
					}
					else {
						controller.sound().right().stop().unwrap();
					}
				}
			}

			_ => ()
		}
	}
}
