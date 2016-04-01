bitflags! {
	flags Button: u32 {
		/// A button has been pressed.
		const A = 0b100000000000000000000000,

		/// B button has been pressed.
		const B = 0b001000000000000000000000,

		/// X button has been pressed.
		const X = 0b010000000000000000000000,

		/// Y button has been pressed.
		const Y = 0b000100000000000000000000,

		/// Pad has been pressed.
		const PAD = 0b000000000000000000000010,

		/// Pad has been touched.
		const PAD_TOUCH = 0b000000000000000000001000,

		/// Analog stick has been pressed.
		const PAD_ANALOG = 0b000000000000000001000000,

		/// Pad down side has been pressed.
		const PAD_DOWN = 0b000000000000100000000000,

		/// Pad left side has been pressed.
		const PAD_LEFT = 0b000000000000010000000000,

		/// Pad right side has been pressed.
		const PAD_RIGHT = 0b000000000000001000000000,

		/// Pad up side has been pressed.
		const PAD_UP = 0b000000000000000100000000,

		/// Trackpad has been pressed.
		const TRACK = 0b000000000000000000000100,

		/// Trackpad has been touched.
		const TRACK_TOUCH = 0b000000000000000000010000,

		/// Back button has been pressed.
		const BACK = 0b000000000001000000000000,

		/// Home button has been pressed.
		const HOME = 0b000000000010000000000000,

		/// Forward button has been pressed.
		const FORWARD = 0b000000000100000000000000,

		/// Left bumper has been pressed.
		const LEFT_BUMPER = 0b000010000000000000000000,

		/// Right bumper has been pressed.
		const RIGHT_BUMPER = 0b000001000000000000000000,

		/// Left grip has been pressed.
		const LEFT_GRIP = 0b00000001000000000000000,

		/// Right grip has been pressed.
		const RIGHT_GRIP = 0b00000000000000000000001,

		/// Left trigger has been fully pressed.
		const LEFT_TRIGGER = 0b000000100000000000000000,

		/// Right trigger has been fully pressed.
		const RIGHT_TRIGGER = 0b000000010000000000000000,
	}
}
