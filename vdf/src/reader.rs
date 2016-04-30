use std::ops::Deref;
use std::io::{self, Read, BufReader};
use parser::{self, Token};
use nom::IResult::{Done, Incomplete, Error};
use nom::Needed;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Item {
	Statement(String),
	Value(String),
}

impl Into<String> for Item {
	fn into(self) -> String {
		match self {
			Item::Statement(s) => s,
			Item::Value(s)     => s,
		}
	}
}

impl Deref for Item {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		match self {
			&Item::Statement(ref v) =>
				v,

			&Item::Value(ref v) =>
				v,
		}
	}
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Event {
	GroupStart(String),
	GroupEnd,

	Entry(Item, Item),

	End,
}

pub struct Reader<R: Read> {
	stream:   BufReader<R>,
	buffer:   Vec<u8>,
	consumed: usize,
}

impl<R: Read> From<R> for Reader<R> {
	fn from(stream: R) -> Reader<R> {
		Reader {
			stream:   BufReader::new(stream),
			buffer:   Vec::new(),
			consumed: 0,
		}
	}
}

impl<R: Read> Reader<R> {
	fn prepare(&mut self) -> io::Result<()> {
		if self.consumed > 0 {
			self.buffer.drain(..self.consumed);
		}

		loop {
			let needed = match parser::next(&self.buffer) {
				Error(_) =>
					return Err(io::Error::new(io::ErrorKind::Other, "parse error")),

				Incomplete(Needed::Size(size)) =>
					size,

				Incomplete(Needed::Unknown) =>
					64,

				Done(rest, _) => {
					self.consumed = self.buffer.len() - rest.len();
					break;
				}
			};

			if try!(self.stream.by_ref().take(needed as u64).read_to_end(&mut self.buffer)) == 0 {
				return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected eof"));
			}
		}

		Ok(())
	}

	pub fn token(&mut self) -> io::Result<Token> {
		try!(self.prepare());

		match parser::next(&self.buffer) {
			Done(_, token) =>
				Ok(token),

			_ =>
				unreachable!()
		}
	}

	pub fn event(&mut self) -> io::Result<Event> {
		let key = match self.token() {
			Err(ref err) if err.kind() == io::ErrorKind::UnexpectedEof =>
				return Ok(Event::End),

			Err(err) =>
				return Err(err),

			Ok(Token::GroupEnd) =>
				return Ok(Event::GroupEnd),

			Ok(Token::GroupStart) =>
				return Err(io::Error::new(io::ErrorKind::Other, "unexpected token")),

			Ok(Token::Item(s)) =>
				Item::Value(s.into_owned()),

			Ok(Token::Statement(s)) =>
				Item::Statement(s.into_owned()),
		};

		let value = match self.token() {
			Err(ref err) if err.kind() == io::ErrorKind::UnexpectedEof =>
				return Ok(Event::End),

			Err(err) =>
				return Err(err),

			Ok(Token::GroupEnd) =>
				return Err(io::Error::new(io::ErrorKind::Other, "unexpected token")),

			Ok(Token::GroupStart) =>
				return Ok(Event::GroupStart(key.into())),

			Ok(Token::Item(s)) =>
				Item::Value(s.into_owned()),

			Ok(Token::Statement(s)) =>
				Item::Statement(s.into_owned()),
		};

		Ok(Event::Entry(key, value))
	}
}

impl<R: Read> Iterator for Reader<R> {
	type Item = Event;

	fn next(&mut self) -> Option<Self::Item> {
		match self.event() {
			Ok(Event::End) =>
				None,

			Ok(event) =>
				Some(event),

			Err(..) =>
				None
		}
	}
}
