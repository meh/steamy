use std::fmt;
use std::error;
use std::io;

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	Parse,
	Eof,
}

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		Error::Io(value)
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

			&Error::Parse =>
				"Parsing error.",

			&Error::Eof =>
				"EOF reached.",
		}
	}
}
