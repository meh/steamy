use vdf;
use {Result as Res, Error};
use super::Input;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Binding {
	Key(Key),
	KeyPad(KeyPad),
	Mouse(Mouse),
	XBox(XBox),
	Action(Action),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Key {
	Esc,

	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,

	BackTick,
	_1,
	_2,
	_3,
	_4,
	_5,
	_6,
	_7,
	_8,
	_9,
	_0,
	Minus,
	Equal,
	BackSpace,

	Tab,
	Q,
	W,
	E,
	R,
	T,
	Y,
	U,
	I,
	O,
	P,
	OpenBracket,
	CloseBracket,

	CapsLock,
	A,
	S,
	D,
	F,
	G,
	H,
	J,
	K,
	L,
	SemiColon,
	Quote,
	Enter,
	
	LeftShift,
	Z,
	X,
	C,
	V,
	B,
	N,
	M,
	Comma,
	Dot,
	Slash,
	RightShift,

	LeftControl,
	Meta,
	LeftAlt,
	Space,
	RightAlt,
	RightControl,

	VolumeUp,
	VolumeDown,
	Mute,
	Play,
	Stop,
	NextTrack,
	PreviousTrack,

	Insert,
	Home,
	PageUp,
	Delete,
	End,
	PageDown,

	Up,
	Down,
	Left,
	Right,
}

impl Into<Binding> for Key {
	fn into(self) -> Binding {
		Binding::Key(self)
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyPad {
	NumLock,
	Slash,
	Asterisk,
	Minus,

	_7,
	_8,
	_9,
	Plus,

	_4,
	_5,
	_6,

	_1,
	_2,
	_3,
	Enter,

	_0,
	Dot,
}

impl Into<Binding> for KeyPad {
	fn into(self) -> Binding {
		Binding::KeyPad(self)
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum XBox {
	LT,
	LB,
	LS,

	RT,
	RB,
	RS,

	X,
	Y,
	A,
	B,

	Back,
	Forward,

	Up,
	Down,
	Left,
	Right,
}

impl Into<Binding> for XBox {
	fn into(self) -> Binding {
		Binding::XBox(self)
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mouse {
	Left,
	Middle,
	Right,

	ScrollUp,
	ScrollDown,

	_4,
	_5,
}

impl Into<Binding> for Mouse {
	fn into(self) -> Binding {
		Binding::Mouse(self)
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
	ShowKeyboard,
	ChangePreset(u32, u32, u32),
	ModeShift(Input, u32)
}

impl Into<Binding> for Action {
	fn into(self) -> Binding {
		Binding::Action(self)
	}
}

impl Binding {
	pub fn load(entry: &vdf::Entry) -> Res<Self> {
		let (group, item) = {
			let matches = ok!(entry.as_str())?.splitn(2, ' ').collect::<Vec<&str>>();
			let group   = ok!(matches.get(0))?.to_lowercase();
			let matches = ok!(matches.get(1))?.splitn(2, ", ").collect::<Vec<&str>>();
			let item    = ok!(matches.get(0))?.to_uppercase();
			let _desc   = matches.get(1).map(|s| s.trim().to_owned());

			(group, item)
		};

		Ok(match &*group {
			"key_press" => match &*item {
				"ESCAPE" => Key::Esc.into(),

				"F1"  => Key::F1.into(),
				"F2"  => Key::F2.into(),
				"F3"  => Key::F3.into(),
				"F4"  => Key::F4.into(),
				"F5"  => Key::F5.into(),
				"F6"  => Key::F6.into(),
				"F7"  => Key::F7.into(),
				"F8"  => Key::F8.into(),
				"F9"  => Key::F9.into(),
				"F10" => Key::F10.into(),
				"F11" => Key::F11.into(),
				"F12" => Key::F12.into(),

				"BACK_TICK" => Key::BackTick.into(),
				"1"         => Key::_1.into(),
				"2"         => Key::_2.into(),
				"3"         => Key::_3.into(),
				"4"         => Key::_4.into(),
				"5"         => Key::_5.into(),
				"6"         => Key::_6.into(),
				"7"         => Key::_7.into(),
				"8"         => Key::_8.into(),
				"9"         => Key::_9.into(),
				"0"         => Key::_0.into(),
				"DASH"      => Key::Minus.into(),
				"EQUALS"    => Key::Equal.into(),
				"BACKSPACE" => Key::BackSpace.into(),

				"TAB"           => Key::Tab.into(),
				"Q"             => Key::Q.into(),
				"W"             => Key::W.into(),
				"E"             => Key::E.into(),
				"R"             => Key::R.into(),
				"T"             => Key::T.into(),
				"Y"             => Key::Y.into(),
				"U"             => Key::U.into(),
				"I"             => Key::I.into(),
				"O"             => Key::O.into(),
				"P"             => Key::P.into(),
				"LEFT_BRACKET"  => Key::OpenBracket.into(),
				"RIGHT_BRACKET" => Key::CloseBracket.into(),

				"CAPSLOCK"         => Key::CapsLock.into(),
				"A"                => Key::A.into(),
				"S"                => Key::S.into(),
				"D"                => Key::D.into(),
				"F"                => Key::F.into(),
				"G"                => Key::G.into(),
				"H"                => Key::H.into(),
				"J"                => Key::J.into(),
				"K"                => Key::K.into(),
				"L"                => Key::L.into(),
				"SEMICOLON"        => Key::SemiColon.into(),
				"SINGLE_QUOTE"     => Key::Quote.into(),
				"RETURN" | "ENTER" => Key::Enter.into(),

				"LEFT_SHIFT"    => Key::LeftShift.into(),
				"Z"             => Key::Z.into(),
				"X"             => Key::X.into(),
				"C"             => Key::C.into(),
				"V"             => Key::V.into(),
				"B"             => Key::B.into(),
				"N"             => Key::N.into(),
				"M"             => Key::M.into(),
				"COMMA"         => Key::Comma.into(),
				"PERIOD"        => Key::Dot.into(),
				"FORWARD_SLASH" => Key::Slash.into(),
				"RIGHT_SHIFT"   => Key::RightShift.into(),

				"LEFT_CONTROL"  => Key::LeftControl.into(),
				"LEFT_WINDOWS"  => Key::Meta.into(),
				"LEFT_ALT"      => Key::LeftAlt.into(),
				"SPACE"         => Key::Space.into(),
				"RIGHT_ALT"     => Key::RightAlt.into(),
				"RIGHT_CONTROL" => Key::RightControl.into(),

				"VOLUME_UP"   => Key::VolumeUp.into(),
				"VOLUME_DOWN" => Key::VolumeDown.into(),
				"MUTE"        => Key::Mute.into(),
				"PLAY"        => Key::Play.into(),
				"STOP"        => Key::Stop.into(),
				"NEXT_TRACK"  => Key::NextTrack.into(),
				"PREV_TRACK"  => Key::PreviousTrack.into(),

				"INSERT"    => Key::Insert.into(),
				"HOME"      => Key::Home.into(),
				"PAGE_UP"   => Key::PageUp.into(),
				"DELETE"    => Key::Delete.into(),
				"END"       => Key::End.into(),
				"PAGE_DOWN" => Key::PageDown.into(),

				"UP_ARROW"    => Key::Up.into(),
				"DOWN_ARROW"  => Key::Down.into(),
				"RIGHT_ARROW" => Key::Right.into(),
				"LEFT_ARROW"  => Key::Left.into(),

				"NUM_LOCK"             => KeyPad::NumLock.into(),
				"KEYPAD_FORWARD_SLASH" => KeyPad::Slash.into(),
				"KEYPAD_ASTERISK"      => KeyPad::Asterisk.into(),
				"KEYPAD_DASH"          => KeyPad::Minus.into(),

				"KEYPAD_7"    => KeyPad::_7.into(),
				"KEYPAD_8"    => KeyPad::_8.into(),
				"KEYPAD_9"    => KeyPad::_9.into(),
				"KEYPAD_PLUS" => KeyPad::Plus.into(),

				"KEYPAD_4" => KeyPad::_4.into(),
				"KEYPAD_5" => KeyPad::_5.into(),
				"KEYPAD_6" => KeyPad::_6.into(),

				"KEYPAD_1"     => KeyPad::_1.into(),
				"KEYPAD_2"     => KeyPad::_2.into(),
				"KEYPAD_3"     => KeyPad::_3.into(),
				"KEYPAD_ENTER" => KeyPad::Enter.into(),

				"KEYPAD_0"      => KeyPad::_0.into(),
				"KEYPAD_PERIOD" => KeyPad::Dot.into(),

				_ =>
					return Err(Error::NotSupported)
			},

			"xinput_button" => match &*item {
				"TRIGGER_LEFT"  => XBox::LT,
				"SHOULDER_LEFT" => XBox::LB,
				"JOYSTICK_LEFT" => XBox::LS,

				"TRIGGER_RIGHT"  => XBox::RT,
				"SHOULDER_RIGHT" => XBox::RB,
				"JOYSTICK_RIGHT" => XBox::RS,

				"A" => XBox::A,
				"B" => XBox::B,
				"X" => XBox::X,
				"Y" => XBox::Y,

				"DPAD_UP"    => XBox::Up,
				"DPAD_DOWN"  => XBox::Down,
				"DPAD_RIGHT" => XBox::Right,
				"DPAD_LEFT"  => XBox::Left,

				_ =>
					return Err(Error::NotSupported)
			}.into(),

			"mouse_button" => match &*item {
				"LEFT"   => Mouse::Left,
				"MIDDLE" => Mouse::Middle,
				"RIGHT"  => Mouse::Right,

				_ =>
					return Err(Error::NotSupported)
			}.into(),

			"mouse_wheel" => match &*item {
				"SCROLL_DOWN" => Mouse::ScrollDown,
				"SCROLL_UP"   => Mouse::ScrollUp,

				_ =>
					return Err(Error::NotSupported)
			}.into(),

			"controller_action" => {
				let matches = item.splitn(2, ' ').collect::<Vec<&str>>();
				let item    = ok!(matches.get(0))?;
				let args    = matches.get(1).map(|s| *s).unwrap_or("");

				match *item {
					"SHOW_KEYBOARD" => Action::ShowKeyboard,
					"CHANGE_PRESET" => {
						let matches = args.split(' ').collect::<Vec<&str>>();
						let foo     = ok!(matches.get(0))?.parse::<u32>().map_err(|_| Error::NotSupported)?;
						let bar     = ok!(matches.get(1))?.parse::<u32>().map_err(|_| Error::NotSupported)?;
						let baz     = ok!(matches.get(2))?.parse::<u32>().map_err(|_| Error::NotSupported)?;

						Action::ChangePreset(foo, bar, baz)
					}

					_ =>
						return Err(Error::NotSupported)
				}
			}.into(),

			"mode_shift" => {
				let matches = item.splitn(2, ' ').collect::<Vec<&str>>();
				let input   = Input::parse(ok!(matches.get(0))?)?;
				let id      = ok!(matches.get(1))?.parse::<u32>().map_err(|_| Error::InvalidParameter)?;

				Action::ModeShift(input, id)
			}.into(),

			_ =>
				return Err(Error::NotSupported)
		})
	}
}
