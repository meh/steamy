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
		self.controller.settings().sensors = false;
		self.controller.reset()
	}

	/// Turn the sensors on.
	pub fn on(self) -> Res<()> {
		self.controller.settings().sensors = true;
		self.controller.reset()
	}
}
