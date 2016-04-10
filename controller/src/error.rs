use std::io;

#[cfg(target_os = "linux")]
use usb;

#[cfg(not(target_os = "linux"))]
use hid as usb;

/// Controller error.
#[derive(Debug)]
pub enum Error {
	/// An IO error.
	Io(io::Error),

	/// An USB error.
	Usb(usb::Error),

	/// Invalid parameter.
	InvalidParameter,

	/// Not supported.
	NotSupported,
}

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		Error::Io(value)
	}
}

impl From<usb::Error> for Error {
	fn from(value: usb::Error) -> Self {
		Error::Usb(value)
	}
}
