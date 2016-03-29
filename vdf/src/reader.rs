use std::io::{self, Read, BufRead, BufReader};
use parser::{self, Event};
use nom::IResult::{Done, Incomplete, Error};
use nom::Needed;

pub struct Reader<R: Read> {
	stream: BufReader<R>,
	buffer: Vec<u8>,
}

impl<R: Read> From<R> for Reader<R> {
	fn from(stream: R) -> Reader<R> {
		Reader {
			stream: BufReader::new(stream),
			buffer: Vec::new(),
		}
	}
}

impl<R: Read> Reader<R> {
	pub fn event(&mut self) -> io::Result<Event<'static>> {
		let event;
		let consumed;

		loop {
			let needed = match parser::next(&self.buffer) {
				Error(err) =>
					return Err(io::Error::new(io::ErrorKind::Other, err)),

				Incomplete(Needed::Size(needed)) =>
					needed,

				Incomplete(Needed::Unknown) =>
					64,

				Done(rest, ev) => {
					consumed = self.buffer.len() - rest.len();
					event    = ev.to_owned();

					break;
				}
			};

			try!(self.stream.by_ref().take(needed as u64).read_to_end(&mut self.buffer));
		}

		self.buffer.drain(..consumed);

		Ok(event)
	}
}
