use std::io::Write;
use {Result as Res, Controller};

/// Controller sensors management.
pub struct Sensors<'a, 'b: 'a> {
	controller: &'a mut Controller<'b>,
}

impl<'a, 'b> Sensors<'a, 'b> {
	#[doc(hidden)]
	pub fn new(controller: &'a mut Controller<'b>) -> Sensors<'a, 'b> {
		Sensors {
			controller: controller,
		}
	}

	/// Turn the sensors off.
	pub fn off(self) -> Res<()> {
		try!(self.controller.control(|mut buf| {
			try!(buf.write(&[
				0x87, 0x15, 0x32, 0x84,
				0x03, 0x18, 0x00, 0x00,
				0x31, 0x02, 0x00, 0x08,
				0x07, 0x00, 0x07, 0x07,
				0x00, 0x30, 0x00, 0x00,
				0x2f, 0x01
			][..]));

			Ok(())
		}));

		Ok(())
	}

	/// Turn the sensors on.
	pub fn on(self) -> Res<()> {
		try!(self.controller.control(|mut buf| {
			try!(buf.write(&[
				0x87, 0x15, 0x32, 0x84,
				0x03, 0x18, 0x00, 0x00,
				0x31, 0x02, 0x00, 0x08,
				0x07, 0x00, 0x07, 0x07,
				0x00, 0x30, 0x14, 0x00,
				0x2f, 0x01
			][..]));

			Ok(())
		}));

		Ok(())
	}
}
