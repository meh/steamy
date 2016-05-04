use std::io::{Read, Seek, SeekFrom};
use byteorder::{ReadBytesExt, BigEndian, LittleEndian};

use {Result as Res, Error, Button};

/// The controller state.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
	/// The controller is powering on or off.
	Power(bool),

	Idle {
		/// Sequence number for the state.
		sequence: u32,
	},

	Input {
		/// Sequence number for the state.
		sequence: u32,

		/// Button state of the controller.
		buttons: Button,

		/// Trigger state of the controller.
		trigger: Trigger,

		/// Pads state.
		pad: Pad,

		/// Orientation of the controller if sensors are enabled.
		orientation: Angles,

		/// Acceleration of the controller if sensors are enabled.
		acceleration: Angles,
	}
}

/// The pressure on the triggers of the controller.
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Trigger {
	/// The left trigger.
	pub left: f32,

	/// The right trigger.
	pub right: f32,
}

/// The pads of the controller.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Pad {
	/// The left pad.
	pub left: Axis,

	/// The right pad.
	pub right: Axis,
}

/// Axis on the pad.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Axis {
	/// The X axis.
	pub x: i16,

	/// The Y axis.
	pub y: i16,
}

impl Axis {
	pub fn is_empty(&self) -> bool {
		self.x == 0 && self.y == 0
	}
}

/// 3D position of the controller.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Angles {
	/// The pitch.
	pub pitch: i16,

	/// The roll.
	pub roll: i16,

	/// The yaw.
	pub yaw: i16,
}

impl State {
	/// Parse the state from a given packet.
	pub fn parse<R: Read + Seek>(id: u8, mut buffer: R) -> Res<State> {
		match id {
			0x01 => {
				let sequence = try!(buffer.read_u32::<LittleEndian>());

				let buttons = try!(buffer.read_u32::<BigEndian>());
				let ltrig   = buttons & 0xff;
				let buttons = buttons >> 8;
				let rtrig   = try!(buffer.read_u8());

				try!(buffer.seek(SeekFrom::Current(3)));

				let lpad_x = try!(buffer.read_i16::<LittleEndian>());
				let lpad_y = try!(buffer.read_i16::<LittleEndian>());
				let rpad_x = try!(buffer.read_i16::<LittleEndian>());
				let rpad_y = try!(buffer.read_i16::<LittleEndian>());

				try!(buffer.seek(SeekFrom::Current(10)));

				let apitch = try!(buffer.read_i16::<LittleEndian>());
				let ayaw   = try!(buffer.read_i16::<LittleEndian>());
				let aroll  = try!(buffer.read_i16::<LittleEndian>());

				let opitch = try!(buffer.read_i16::<LittleEndian>());
				let oyaw   = try!(buffer.read_i16::<LittleEndian>());
				let oroll  = try!(buffer.read_i16::<LittleEndian>());

				Ok(State::Input {
					sequence: sequence,

					buttons: try!(Button::from_bits(buttons).ok_or(Error::InvalidParameter)),

					trigger: Trigger {
						left:  ltrig as f32 / 255.0,
						right: rtrig as f32 / 255.0,
					},

					pad: Pad {
						left: Axis {
							x: lpad_x,
							y: lpad_y,
						},

						right: Axis {
							x: rpad_x,
							y: rpad_y,
						},
					},

					orientation: Angles {
						roll:  oroll,
						pitch: opitch,
						yaw:   oyaw,
					},

					acceleration: Angles {
						roll:  aroll,
						pitch: apitch,
						yaw:   ayaw,
					}
				})
			}

			0x03 => {
				let mode = try!(buffer.read_u8());

				Ok(State::Power(match mode {
					0x01 => false,
					0x02 => true,

					_ =>
						return Err(Error::InvalidParameter)
				}))
			}

			0x04 => {
				let sequence = try!(buffer.read_u32::<LittleEndian>());

				Ok(State::Idle {
					sequence: sequence,
				})
			}

			_ =>
				Err(Error::InvalidParameter)
		}
	}
}
