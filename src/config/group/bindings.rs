use vdf;
use {Result as Res};
use config::Binding;
use config::group::{Mode};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Bindings {
	FourButtons {
		a: Option<Vec<Binding>>,
		b: Option<Vec<Binding>>,
		x: Option<Vec<Binding>>,
		y: Option<Vec<Binding>>,
	},

	DPad {
		north: Option<Vec<Binding>>,
		south: Option<Vec<Binding>>,
		east:  Option<Vec<Binding>>,
		west:  Option<Vec<Binding>>,
		click: Option<Vec<Binding>>,
	},

	AbsoluteMouse {
		click:  Option<Vec<Binding>>,
		double: Option<Vec<Binding>>,
	},

	Trigger {
		click: Option<Vec<Binding>>,
	},

	ScrollWheel {
		cw:    Option<Vec<Binding>>,
		ccw:   Option<Vec<Binding>>,
		click: Option<Vec<Binding>>,
	},

	MouseJoystick {
		click: Option<Vec<Binding>>,
	},

	JoystickMove {
		click: Option<Vec<Binding>>,
	},

	TouchMenu {
		buttons: Vec<Option<Vec<Binding>>>,
	}
}

impl Bindings {
	pub fn empty(mode: Mode) -> Self {
		match mode {
			Mode::FourButtons => {
				Bindings::FourButtons {
					a: None,
					b: None,
					x: None,
					y: None,
				}
			}

			Mode::DPad => {
				Bindings::DPad {
					north: None,
					south: None,
					east:  None,
					west:  None,
					click: None,
				}
			}

			Mode::AbsoluteMouse => {
				Bindings::AbsoluteMouse {
					click:  None,
					double: None,
				}
			}

			Mode::Trigger => {
				Bindings::Trigger {
					click: None,
				}
			}

			Mode::ScrollWheel => {
				Bindings::ScrollWheel {
					cw:    None,
					ccw:   None,
					click: None,
				}
			}

			Mode::MouseJoystick => {
				Bindings::MouseJoystick {
					click: None,
				}
			}

			Mode::JoystickMove => {
				Bindings::JoystickMove {
					click: None,
				}
			}

			Mode::TouchMenu => {
				Bindings::TouchMenu {
					buttons: Vec::new(),
				}
			}
		}
	}

	pub fn load(mode: Mode, table: &vdf::Entry) -> Res<Self> {
		Ok(match mode {
			Mode::FourButtons => {
				Bindings::FourButtons {
					a: binding!(table@button_A)?,
					b: binding!(table@button_B)?,
					x: binding!(table@button_X)?,
					y: binding!(table@button_Y)?,
				}
			}

			Mode::DPad => {
				Bindings::DPad {
					north: binding!(table@dpad_north)?,
					south: binding!(table@dpad_south)?,
					east:  binding!(table@dpad_east)?,
					west:  binding!(table@dpad_west)?,
					click: binding!(table@click)?,
				}
			}

			Mode::AbsoluteMouse => {
				Bindings::AbsoluteMouse {
					click:  binding!(table@click)?,
					double: binding!(table@double_tap)?,
				}
			}

			Mode::Trigger => {
				Bindings::Trigger {
					click: binding!(table@click)?,
				}
			}

			Mode::ScrollWheel => {
				Bindings::ScrollWheel {
					cw:    binding!(table@scroll_clockwise)?,
					ccw:   binding!(table@scroll_counterclockwise)?,
					click: binding!(table@click)?,
				}
			}

			Mode::MouseJoystick => {
				Bindings::MouseJoystick {
					click: binding!(table@click)?,
				}
			}

			Mode::JoystickMove => {
				Bindings::JoystickMove {
					click: binding!(table@click)?,
				}
			}

			Mode::TouchMenu => {
				let mut buttons = Vec::new();
				for i in 0 .. {
					if let Some(binding) = binding!(table, format!("touch_menu_button_{}", i))? {
						buttons.push(Some(binding));
					}
					else {
						break;
					}
				}

				Bindings::TouchMenu {
					buttons: buttons,
				}
			}
		})
	}
}
