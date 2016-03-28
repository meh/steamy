use std::io;
use usb;
use byteorder;

/// Controller error.
#[derive(Debug)]
pub enum Error {
	/// An IO error.
	Io(io::Error),

	/// An USB error.
	Usb(usb::Error),

	/// A byteorder error.
	ByteOrder(byteorder::Error),
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

impl From<byteorder::Error> for Error {
	fn from(value: byteorder::Error) -> Self {
		Error::ByteOrder(value)
	}
}
