use std::time::Instant;
use std::collections::HashSet;
use uinput;
use {Result as Res};
use input;
use config::{self, Config, preset};
use super::{ButtonDiamond, PadLeft, PadRight, TriggerLeft, TriggerRight};
use super::switch::{self, Switch};

pub struct Preset<'a> {
	pub id:     u32,
	pub config: &'a config::Preset,

	pub switch:         Switch<'a>,
	pub button_diamond: ButtonDiamond<'a>,
	pub pad_left:       PadLeft<'a>,
	pub pad_right:      PadRight<'a>,
	pub trigger_left:   TriggerLeft<'a>,
	pub trigger_right:  TriggerRight<'a>,
}

impl<'a> Preset<'a> {
	pub fn load(id: u32, config: &Config) -> Res<Preset> {
		let preset = config.presets.get(&id).unwrap();

		Ok(Preset {
			id:     id,
			config: preset,

			switch: Switch::load(preset)?,

			button_diamond: ButtonDiamond::load(
				group_by!(config, id, config::Input::ButtonDiamond, true, false),
				group_by!(config, id, config::Input::ButtonDiamond, true, true))?,

			pad_left: PadLeft::load(
				group_by!(config, id, config::Input::PadLeft, true, false),
				group_by!(config, id, config::Input::PadLeft, true, true))?,

			pad_right: PadRight::load(
				group_by!(config, id, config::Input::PadRight, true, false),
				group_by!(config, id, config::Input::PadRight, true, true))?,

			trigger_left: TriggerLeft::load(
				group_by!(config, id, config::Input::TriggerLeft, true, false),
				group_by!(config, id, config::Input::TriggerLeft, true, true))?,

			trigger_right: TriggerRight::load(
				group_by!(config, id, config::Input::TriggerRight, true, false),
				group_by!(config, id, config::Input::TriggerRight, true, true))?,
		})
	}

	pub fn handles(&self, button: input::Button) -> bool {
		if let Some(button) = switch::convert(button) {
			self.config.bindings.contains_key(&button)
		}
		else {
			false
		}
	}

	pub fn shift(&mut self, id: u32) {

	}
}
