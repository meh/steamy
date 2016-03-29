//! Steam controller handling library.

extern crate byteorder;
#[macro_use] extern crate bitflags;

#[cfg(target_os = "linux")]
extern crate libusb as usb;

#[cfg(target_os = "windows")]
extern crate hid;

const VENDOR_ID:  u16       = 0x28de;
const PRODUCT_ID: [u16;  2] = [0x1102, 0x1142];
const ENDPOINT:   [u8;   2] = [3, 2];
const INDEX:      [u16;  2] = [2, 1];

mod error;
pub use error::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

mod manager;
pub use manager::Manager;

mod controller;
pub use controller::Controller;

mod feedback;
pub use feedback::Feedback;

mod sensors;
pub use sensors::Sensors;

pub mod button;
pub use button::Button;

mod state;
pub use state::{State, Axis, Trigger};
