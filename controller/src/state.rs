use std::io::{Read, Seek, SeekFrom};
use byteorder::{ReadBytesExt, BigEndian, LittleEndian};
use usb;

use super::Result as Res;
use super::Button;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
	Power(bool),

	Idle {
		sequence: u32,
	},

	Input {
		sequence: u32,

		buttons: Button,
		trigger: Trigger,
		pad:     Pad,

		orientation:  Angles,
		acceleration: Angles,
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Trigger {
	pub left:  Precision,
	pub right: Precision,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Precision {
	pub low:  f32,
	pub high: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pad {
	pub left:  Axis,
	pub right: Axis,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Axis {
	pub x: i16,
	pub y: i16,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Angles {
	pub pitch: i16,
	pub roll:  i16,
	pub yaw:   i16,
}

impl State {
	pub fn parse<R: Read + Seek>(mut buffer: R) -> Res<State> {
		try!(buffer.seek(SeekFrom::Current(2)));
		let status = try!(buffer.read_u16::<BigEndian>());

		match status {
			0x0301 => {
				let mode = try!(buffer.read_u8());

				Ok(State::Power(match mode {
					0x01 => false,
					0x02 => true,
					_    => return Err(usb::Error::InvalidParam.into()),
				}))
			}

			0x040b => {
				let sequence = try!(buffer.read_u32::<LittleEndian>());

				Ok(State::Idle {
					sequence: sequence,
				})
			}

			0x013c => {
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

				let ltrigp = try!(buffer.read_u16::<LittleEndian>());
				let rtrigp = try!(buffer.read_u16::<LittleEndian>());

				let oroll  = try!(buffer.read_i16::<LittleEndian>());
				let oyaw   = try!(buffer.read_i16::<LittleEndian>());
				let opitch = try!(buffer.read_i16::<LittleEndian>());

				let aroll  = try!(buffer.read_i16::<LittleEndian>());
				let ayaw   = try!(buffer.read_i16::<LittleEndian>());
				let apitch = try!(buffer.read_i16::<LittleEndian>());

				Ok(State::Input {
					sequence: sequence,

					buttons: try!(Button::from_bits(buttons).ok_or(usb::Error::InvalidParam)),

					trigger: Trigger {
						left: Precision {
							low: ltrig as f32 / 255.0,
							high: ltrigp as f64 / 32767.0,
						},

						right: Precision {
							low: rtrig as f32 / 255.0,
							high: rtrigp as f64 / 32767.0,
						}
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

			_ =>
				Err(usb::Error::InvalidParam.into())
		}
	}
}
