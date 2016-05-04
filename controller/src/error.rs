use std::fmt;
use std::error;
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

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		f.write_str(error::Error::description(self))
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match self {
			&Error::Io(ref err) =>
				err.description(),

			&Error::Usb(ref err) =>
				err.description(),

			&Error::InvalidParameter =>
				"Invalid parameter.",

			&Error::NotSupported =>
				"Unsupported."
		}
	}
}
