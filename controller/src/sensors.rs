use std::time::Duration;
use byteorder::{WriteBytesExt, BigEndian};

use super::Result as Res;
use super::Controller;

pub struct Sensors<'a, 'b: 'a> {
	controller: &'a mut Controller<'b>,
}

impl<'a, 'b> Sensors<'a, 'b> {
	pub fn new(controller: &'a mut Controller<'b>) -> Sensors<'a, 'b> {
		Sensors {
			controller: controller,
		}
	}

	pub fn off(self) -> Res<()> {
		try!(self.controller.control(Duration::from_secs(0), |mut buf| {
			try!(buf.write_u32::<BigEndian>(0x87153284));
			try!(buf.write_u32::<BigEndian>(0x03180000));
			try!(buf.write_u32::<BigEndian>(0x31020008));
			try!(buf.write_u32::<BigEndian>(0x07000707));
			try!(buf.write_u32::<BigEndian>(0x00300000));
			try!(buf.write_u32::<BigEndian>(0x2f010000));

			Ok(())
		}));

		Ok(())
	}

	pub fn on(self) -> Res<()> {
		try!(self.controller.control(Duration::from_secs(0), |mut buf| {
			try!(buf.write_u32::<BigEndian>(0x87153284));
			try!(buf.write_u32::<BigEndian>(0x03180000));
			try!(buf.write_u32::<BigEndian>(0x31020008));
			try!(buf.write_u32::<BigEndian>(0x07000707));
			try!(buf.write_u32::<BigEndian>(0x00301400));
			try!(buf.write_u32::<BigEndian>(0x2f010000));

			Ok(())
		}));

		Ok(())
	}
}
