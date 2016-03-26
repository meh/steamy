use std::io;
use usb;
use byteorder;

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	Usb(usb::Error),
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
