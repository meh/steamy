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

	pub button_diamond: ButtonDiamond<'a>,
	pub pad_left:       PadLeft<'a>,
	pub pad_right:      PadRight<'a>,
	pub trigger_left:   TriggerLeft<'a>,
	pub trigger_right:  TriggerRight<'a>,
}

impl<'a> Preset<'a> {
	pub fn load(id: u32, config: &Config) -> Res<Preset> {
		Ok(Preset {
			id:     id,
			config: config.presets.get(&id).unwrap(),

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

	pub fn switch<'b>(&'b mut self) -> Switch<'b, 'a> where 'a: 'b {
		Switch::new(self)
	}

	pub fn button_diamond<'b>(&'b mut self) -> &'b mut ButtonDiamond<'a> where 'a: 'b {
		&mut self.button_diamond
	}

	pub fn pad_left<'b>(&'b mut self) -> &'b mut PadLeft<'a> where 'a: 'b {
		&mut self.pad_left
	}

	pub fn pad_right<'b>(&'b mut self) -> &'b mut PadRight<'a> where 'a: 'b {
		&mut self.pad_right
	}

	pub fn trigger_left<'b>(&'b mut self) -> &'b mut TriggerLeft<'a> where 'a: 'b {
		&mut self.trigger_left
	}

	pub fn trigger_right<'b>(&'b mut self) -> &'b mut TriggerRight<'a> where 'a: 'b {
		&mut self.trigger_right
	}
}
