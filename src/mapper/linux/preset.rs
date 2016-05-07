use {Result as Res};
use config::{self, Config};
use super::{ButtonDiamond, PadLeft, PadRight, TriggerLeft, TriggerRight};

pub struct Preset<'a> {
	pub id: u32,

	pub button_diamond: ButtonDiamond<'a>,
	pub pad_left:       PadLeft<'a>,
	pub pad_right:      PadRight<'a>,
	pub trigger_left:   TriggerLeft<'a>,
	pub trigger_right:  TriggerRight<'a>,
}

impl<'a> Preset<'a> {
	pub fn load(id: u32, config: &Config) -> Res<Preset> {
		Ok(Preset {
			id: id,

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
}
