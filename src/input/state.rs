use controller::{self, button};
use super::{Event, Button, Trigger, Pad};

#[derive(Debug)]
pub struct State {
	buttons:      controller::Button,
	trigger:      controller::Trigger,
	pad:          controller::Pad,
	orientation:  controller::Angles,
	acceleration: controller::Angles,
}

impl Default for State {
	fn default() -> Self {
		State {
			buttons:      controller::Button::empty(),
			trigger:      Default::default(),
			pad:          Default::default(),
			orientation:  Default::default(),
			acceleration: Default::default(),
		}
	}
}

impl State {
	pub fn update(&mut self, state: controller::State) -> Vec<Event> {
		let mut events = Vec::new();

		match state {
			controller::State::Power(true) => {
				events.push(Event::Connected);
			}

			controller::State::Power(false) => {
				events.push(Event::Disconnected);
			}

			controller::State::Input { buttons, trigger, pad, orientation, acceleration, .. } => {
				button!(events, self.buttons, buttons, {
					button::A => Button::A,
					button::B => Button::B,
					button::X => Button::X,
					button::Y => Button::Y,

					button::PAD_DOWN  => Button::Down,
					button::PAD_LEFT  => Button::Left,
					button::PAD_RIGHT => Button::Right,
					button::PAD_UP    => Button::Up,

					button::PAD        => Button::Pad,
					button::PAD_TOUCH  => Button::PadTouch,

					button::STICK       => Button::Stick,
					button::STICK_TOUCH => Button::StickTouch,

					button::TRACK       => Button::Track,
					button::TRACK_TOUCH => Button::TrackTouch,

					button::BACK    => Button::Back,
					button::HOME    => Button::Home,
					button::FORWARD => Button::Forward,

					button::LEFT_BUMPER  => Button::BumperLeft,
					button::RIGHT_BUMPER => Button::BumperRight,

					button::LEFT_GRIP  => Button::GripLeft,
					button::RIGHT_GRIP => Button::GripRight,

					button::LEFT_TRIGGER  => Button::TriggerLeft,
					button::RIGHT_TRIGGER => Button::TriggerRight,
				});

				if self.trigger.left != trigger.left {
					events.push(Event::Trigger(Trigger::Left(trigger.left)));
				}

				if self.trigger.right != trigger.right {
					events.push(Event::Trigger(Trigger::Right(trigger.right)));
				}

				if self.pad.left != pad.left {
					events.push(Event::Pad(Pad::Left(pad.left)));
				}

				if self.pad.right != pad.right {
					events.push(Event::Pad(Pad::Right(pad.right)));
				}

				if self.orientation != orientation {
					events.push(Event::Orientation(orientation));
				}

				if self.acceleration != acceleration {
					events.push(Event::Acceleration(acceleration));
				}

				self.buttons      = buttons;
				self.trigger      = trigger;
				self.pad          = pad;
				self.orientation  = orientation;
				self.acceleration = acceleration;
			}

			controller::State::Idle { .. } =>
				(),
		}

		events
	}
}
