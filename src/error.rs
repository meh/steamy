use std::fmt;
use std::error;
use controller;
use vdf;

#[cfg(target_os = "linux")]
use uinput;

#[derive(Debug)]
pub enum Error {
	Controller(controller::Error),
	Vdf(vdf::Error),

	#[cfg(target_os = "linux")]
	Uinput(uinput::Error),

	NotSupported,
	InvalidParameter,
}

impl From<controller::Error> for Error {
	fn from(value: controller::Error) -> Self {
		Error::Controller(value)
	}
}

impl From<vdf::Error> for Error {
	fn from(value: vdf::Error) -> Self {
		Error::Vdf(value)
	}
}

#[cfg(target_os = "linux")]
impl From<uinput::Error> for Error {
	fn from(value: uinput::Error) -> Self {
		Error::Uinput(value)
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
			&Error::Controller(ref err) =>
				err.description(),

			&Error::Vdf(ref err) =>
				err.description(),

			#[cfg(target_os = "linux")]
			&Error::Uinput(ref err) =>
				err.description(),

			&Error::NotSupported =>
				"Unsupported configuration version.",

			&Error::InvalidParameter =>
				"Invalid parameter in configuration file."
		}
	}
}
