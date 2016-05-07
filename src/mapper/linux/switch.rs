use std::time::Instant;
use std::collections::HashSet;
use std::iter;
use uinput;
use {Result as Res};
use input;
use config::{self, Binding, Config, preset};
use util::iter;
use super::Preset;

pub struct Switch<'b, 'a: 'b> {
	preset: &'b mut Preset<'a>,
}

impl<'a, 'b> Switch<'a, 'b> {
	pub fn new(preset: &'b mut Preset<'a>) -> Switch<'b, 'a> {
		Switch {
			preset: preset,
		}
	}

	pub fn button(&self, device: &mut uinput::Device, _at: Instant, button: input::Button, press: bool) -> Res<HashSet<uinput::Event>> {
		let bindings = if let Some(button) = convert(button) {
			if let Some(bindings) = self.preset.config.bindings.get(&button) {
				iter(bindings.iter())
			}
			else {
				iter(iter::empty())
			}
		}
		else {
			iter(iter::empty())
		};

		let (keys, actions): (Vec<&Binding>, Vec<&Binding>) = bindings.partition(|&binding|
			if let &Binding::Action(..) = binding { false } else { true });

		Ok(keys.iter().map(|&binding|
			device.send(binding, if press { 1 } else { 0 })
				.map(|_| binding.into())).collect()?)
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
