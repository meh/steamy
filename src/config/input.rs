use {Result as Res, Error};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Input {
	A,
	B,
	X,
	Y,

	ButtonEscape,
	ButtonMenu,

	BumperLeft,
	BumperRight,

	TriggerLeft,
	TriggerRight,

	GripLeft,
	GripRight,

	ButtonDiamond,
	TrackpadRight,
	TrackpadLeft,
	Joystick,
	Gyro,
}

impl Input {
	pub fn parse(string: &str) -> Res<Self> {
		Ok(match &*string.to_lowercase() {
			"button_a" => Input::A,
			"button_b" => Input::B,
			"button_x" => Input::X,
			"button_y" => Input::Y,

			"button_diamond" => Input::ButtonDiamond,
			"button_escape" => Input::ButtonEscape,
			"button_menu"   => Input::ButtonMenu,

			"left_bumper"  => Input::BumperLeft,
			"right_bumper" => Input::BumperRight,

			"left_trigger"  => Input::TriggerLeft,
			"right_trigger" => Input::TriggerRight,

			"button_back_left"  => Input::GripLeft,
			"button_back_right" => Input::GripRight,

			"left_trackpad"  => Input::TrackpadLeft,
			"right_trackpad" => Input::TrackpadRight,
			"joystick"       => Input::Joystick,
			"gyro"           => Input::Gyro,

			_ =>
				return Err(Error::InvalidParameter),
		})
	}
}
