use std::io::Write;
use {Result as Res, Controller};

/// Controller led management.
pub struct Led<'a, 'b: 'a> {
	controller: &'a mut Controller<'b>,
}

impl<'a, 'b> Led<'a, 'b> {
	#[doc(hidden)]
	pub fn new(controller: &'a mut Controller<'b>) -> Led<'a, 'b> {
		Led {
			controller: controller,
		}
	}

	/// Change the led luminosity.
	pub fn level(self, value: u8) -> Res<()> {
		self.controller.control(|mut buf| {
			buf.write(&[
				0x87, 0x03, 0x2d, value
			][..])
		})
	}

	/// Turn the led off.
	pub fn off(self) -> Res<()> {
		self.level(0)
	}

	/// Turn the led on.
	pub fn on(self) -> Res<()> {
		self.level(100)
	}
}
