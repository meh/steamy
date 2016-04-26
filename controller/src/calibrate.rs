use {Result as Res, Controller};

/// Calibration manager.
pub struct Calibrate<'a, 'b: 'a> {
	controller: &'a mut Controller<'b>,
}

impl<'a, 'b> Calibrate<'a, 'b> {
	#[doc(hidden)]
	pub fn new(controller: &'a mut Controller<'b>) -> Calibrate<'a, 'b> {
		Calibrate {
			controller: controller,
		}
	}

	/// Calibrate the trackpads.
	pub fn trackpad(self) -> Res<()> {
		self.controller.control(0xa7)
	}

	/// Calibrate the joystick.
	pub fn joystick(self) -> Res<()> {
		self.controller.control(0xbf)
	}

	/// Calibrate the sensors.
	pub fn sensors(self) -> Res<()> {
		self.controller.control(0xb5)
	}
}
