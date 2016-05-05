use vdf::entry::Parse;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
	FourButtons,
	DPad,
	AbsoluteMouse,
	Trigger,
	ScrollWheel,
	MouseJoystick,
	JoystickMove,
	TouchMenu,
}

impl Parse for Mode {
	fn parse(string: &str) -> Option<Mode> {
		match string {
			"four_buttons"   => Some(Mode::FourButtons),
			"dpad"           => Some(Mode::DPad),
			"absolute_mouse" => Some(Mode::AbsoluteMouse),
			"trigger"        => Some(Mode::Trigger),
			"scrollwheel"    => Some(Mode::ScrollWheel),
			"mouse_joystick" => Some(Mode::MouseJoystick),
			"joystick_move"  => Some(Mode::JoystickMove),
			"touch_menu"     => Some(Mode::TouchMenu),
			_                => None,
		}
	}
}
