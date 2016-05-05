#[macro_use]
mod util;

mod input;
pub use self::input::Input;

mod event;
pub use self::event::{Event, Button, Trigger, Pad};

mod state;
pub use self::state::State;

pub fn spawn() -> Input {
	Input::spawn()
}
