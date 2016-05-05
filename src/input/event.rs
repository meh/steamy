use controller;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Event {
	Connected,
	Disconnected,

	Button(Button, bool),
	Trigger(Trigger),
	Pad(Pad),
	Orientation(controller::Angles),
	Acceleration(controller::Angles),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Button {
	A,
	B,
	X,
	Y,

	Down,
	Left,
	Right,
	Up,

	Pad,
	PadTouch,

	Stick,
	StickTouch,

	Track,
	TrackTouch,

	Back,
	Home,
	Forward,

	BumperLeft,
	BumperRight,

	GripLeft,
	GripRight,

	TriggerLeft,
	TriggerRight,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Trigger {
	Left(f32),
	Right(f32),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pad {
	Left(controller::Axis),
	Right(controller::Axis),
}
