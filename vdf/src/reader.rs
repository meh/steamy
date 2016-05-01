use std::ops::Deref;
use std::io::{Read, BufReader};
use parser::{self, Token};
use {Result as Res, Error};
use nom::IResult::{Done, Incomplete, Error as Fail};
use nom::Needed;

/// Kinds of item.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Item {
	/// A statement, the ones starting with #.
	Statement(String),

	/// A value.
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

/// Reader event.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Event {
	/// A group with the given name is starting.
	GroupStart(String),

	/// A group has ended.
	GroupEnd,

	/// An entry.
	Entry(Item, Item),

	/// EOF has been reached.
	End,
}

/// A streaming VDF reader.
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
	fn prepare(&mut self) -> Res<()> {
		if self.consumed > 0 {
			self.buffer.drain(..self.consumed);
		}

		loop {
			let needed = match parser::next(&self.buffer) {
				Fail(_) =>
					return Err(Error::Parse),

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
				return Err(Error::Eof);
			}
		}

		Ok(())
	}

	/// Get the next parser token without doing any copies.
	pub fn token(&mut self) -> Res<Token> {
		try!(self.prepare());

		match parser::next(&self.buffer) {
			Done(_, token) =>
				Ok(token),

			_ =>
				unreachable!()
		}
	}

	/// Get the next event, this does copies.
	pub fn event(&mut self) -> Res<Event> {
		let key = match self.token() {
			Err(Error::Eof) =>
				return Ok(Event::End),

			Err(err) =>
				return Err(err),

			Ok(Token::GroupEnd) =>
				return Ok(Event::GroupEnd),

			Ok(Token::GroupStart) =>
				return Err(Error::Parse),

			Ok(Token::Item(s)) =>
				Item::Value(s.into_owned()),

			Ok(Token::Statement(s)) =>
				Item::Statement(s.into_owned()),
		};

		let value = match self.token() {
			Err(Error::Eof) =>
				return Ok(Event::End),

			Err(err) =>
				return Err(err),

			Ok(Token::GroupEnd) =>
				return Err(Error::Parse),

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
