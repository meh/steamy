#[macro_use]
extern crate nom;

use std::io::{Read};
use std::fs::File;
use std::path::Path;

mod error;
pub use error::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

pub mod reader;
pub use reader::Reader;

pub mod entry;
pub use entry::{Table, Entry, Statement, Value};

pub mod parser;

/// Create a reader from the given path.
pub fn open<P: AsRef<Path>>(path: P) -> Result<Reader<File>> {
	Ok(Reader::from(try!(File::open(path))))
}

/// Create a reader from the given stream.
pub fn read<R: Read>(stream: R) -> Result<Reader<R>> {
	Ok(Reader::from(stream))
}

/// Load a table from the given path.
pub fn load<P: AsRef<Path>>(path: P) -> Result<Entry> {
	Ok(try!(Table::load(&mut try!(open(path)))).into())
}
