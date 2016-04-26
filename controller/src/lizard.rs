use {Result as Res, Controller};

/// Controller led management.
pub struct Lizard<'a, 'b: 'a> {
	controller: &'a mut Controller<'b>,
}

impl<'a, 'b> Lizard<'a, 'b> {
	#[doc(hidden)]
	pub fn new(controller: &'a mut Controller<'b>) -> Lizard<'a, 'b> {
		Lizard {
			controller: controller,
		}
	}

	/// Enable lizard mode.
	pub fn enable(self) -> Res<()> {
		self.controller.control(0x85)
	}

	/// Disable lizard mode.
	pub fn disable(self) -> Res<()> {
		self.controller.control(0x81)
	}
}
