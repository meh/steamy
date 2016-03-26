use std::time::Duration;
use byteorder::{WriteBytesExt, LittleEndian};

use super::Result as Res;
use super::Controller;

pub struct Feedback<'a, 'b: 'a> {
	controller: &'a mut Controller<'b>,
	side:       u8,
	amplitude:  u16,
	period:     u16,
	count:      u16,
}

impl<'a, 'b> Feedback<'a, 'b> {
	pub fn new(controller: &'a mut Controller<'b>) -> Feedback<'a, 'b> {
		Feedback {
			controller: controller,
			side:       0,
			amplitude:  128,
			period:     0,
			count:      1,
		}
	}

	pub fn left(mut self) -> Self {
		self.side = 1;
		self
	}

	pub fn right(mut self) -> Self {
		self.side = 0;
		self
	}

	pub fn amplitude(mut self, value: u16) -> Self {
		self.amplitude = value;
		self
	}

	pub fn period(mut self, value: u16) -> Self {
		self.period = value;
		self
	}

	pub fn count(mut self, value: u16) -> Self {
		self.count = value;
		self
	}

	pub fn send(self) -> Res<()> {
		let side      = self.side;
		let amplitude = self.amplitude;
		let period    = self.period;
		let count     = self.count;

		try!(self.controller.control(Duration::from_secs(0), |mut buf| {
			try!(buf.write_u8(0x8f));
			try!(buf.write_u8(0x08));
			try!(buf.write_u8(side));
			try!(buf.write_u16::<LittleEndian>(amplitude));
			try!(buf.write_u16::<LittleEndian>(period));
			try!(buf.write_u16::<LittleEndian>(count));

			Ok(())
		}));

		Ok(())
	}
}
