use std::time::Instant;
use std::collections::HashSet;
use std::iter;
use uinput;
use {Result as Res};
use input;
use config::{self, Binding, Config, preset};
use util::iter;
use super::{Preset, Button, util};

pub struct Switch<'a> {
	config: &'a preset::Preset,
}

impl<'a> Switch<'a> {
	pub fn load(config: &'a preset::Preset) -> Res<Switch<'a>> {
		Ok(Switch {
			config: config,
		})
	}
}

impl<'a> Button for Switch<'a> {
	fn button(&mut self, device: &mut uinput::Device, _at: Instant, button: input::Button, press: bool) -> Res<HashSet<&Binding>> {
		let bindings = if let Some(button) = convert(button) {
			if let Some(bindings) = self.config.bindings.get(&button) {
				iter(bindings.iter())
			}
			else {
				iter(iter::empty())
			}
		}
		else {
			iter(iter::empty())
		};

		util::button(device, bindings, press)
	}
}

pub fn convert(button: input::Button) -> Option<preset::Button> {
	Some(match button {
		input::Button::A => preset::Button::A,
		input::Button::B => preset::Button::B,
		input::Button::X => preset::Button::X,
		input::Button::Y => preset::Button::Y,

		input::Button::Back    => preset::Button::Back,
		input::Button::Forward => preset::Button::Forward,

		input::Button::BumperLeft  => preset::Button::BumperLeft,
		input::Button::BumperRight => preset::Button::BumperRight,

		input::Button::GripLeft  => preset::Button::GripLeft,
		input::Button::GripRight => preset::Button::GripRight,

		input::Button::TriggerLeft  => preset::Button::TriggerLeft,
		input::Button::TriggerRight => preset::Button::TriggerRight,

		_ => return None,
	})
}
