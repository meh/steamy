use vdf::entry::Parse;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Button {
	A,
	B,
	X,
	Y,

	Back,
	Forward,

	BumperLeft,
	BumperRight,

	GripLeft,
	GripRight,

	TriggerLeft,
	TriggerRight,
}

impl Parse for Button {
	fn parse(string: &str) -> Option<Button> {
		match string {
			"button_A" => Some(Button::A),
			"button_B" => Some(Button::B),
			"button_X" => Some(Button::X),
			"button_Y" => Some(Button::Y),

			"button_escape" => Some(Button::Forward),
			"button_menu"   => Some(Button::Back),

			"right_bumper" => Some(Button::BumperRight),
			"left_bumper"  => Some(Button::BumperLeft),

			"right_trigger" => Some(Button::TriggerRight),
			"left_trigger"  => Some(Button::TriggerLeft),

			"button_back_right" => Some(Button::GripRight),
			"button_back_left"  => Some(Button::GripLeft),

			_ => None,
		}
	}
}
