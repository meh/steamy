use std::str::{self, Utf8Error};
use std::borrow::Cow;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Event<'a> {
	GroupStart,
	GroupEnd,
	Item(Cow<'a, str>),
	Statement(Cow<'a, str>),
}

fn string(buffer: &[u8]) -> Result<Cow<str>, Utf8Error> {
	if buffer.iter().any(|&b| b == b'\\') {
		let mut buffer = buffer.iter().cloned();
		let mut string = Vec::with_capacity(buffer.len());

		while let Some(byte) = buffer.next() {
			if byte == b'\\' {
				match buffer.next() {
					Some(b'\\') => string.push(b'\\'),
					Some(b'n')  => string.push(b'\n'),
					Some(b't')  => string.push(b'\t'),
					Some(b'r')  => string.push(b'\r'),
					Some(b'"')  => string.push(b'"'),
					Some(byte)  => string.extend_from_slice(&[b'\\', byte]),
					None        => break
				}
			}
			else {
				string.push(byte);
			}
		}

		match String::from_utf8(string) {
			Err(err) => Err(err.utf8_error()),
			Ok(str)  => Ok(str.into())
		}
	}
	else {
		Ok(try!(str::from_utf8(buffer)).into())
	}
}

named!(pub next(&[u8]) -> Event,
	chain!(many0!(whitespace) ~ value: alt!(open | close | bare | enclosed) ~ many0!(whitespace),
		|| { value }));

named!(pub whitespace(&[u8]) -> (),
	value!((), alt!(char!(' ') | char!('\t') | char!('\n') | char!('\r'))));

named!(pub open(&[u8]) -> Event,
	value!(Event::GroupStart, char!('{')));

named!(pub close(&[u8]) -> Event,
	value!(Event::GroupEnd, char!('}')));

named!(pub bare(&[u8]) -> Event,
	alt!(bare_statement | bare_item));

named!(bare_statement(&[u8]) -> Event,
	map_res!(chain!(char!('#') ~ value: is_not!(" \t\n\r{}\""), || { value }),
		|v| { string(v).map(|v| Event::Statement(v)) }));

named!(bare_item(&[u8]) -> Event,
	map_res!(is_not!(" \t\n\r{}\""),
		|v| { string(v).map(|v| Event::Item(v)) }));

named!(pub enclosed(&[u8]) -> Event,
	alt!(enclosed_statement | enclosed_item));

named!(enclosed_content,
	escaped!(is_not!("\"\\"), '\\', is_a_bytes!(&b"\"n\\"[..])));

named!(enclosed_statement(&[u8]) -> Event,
	map_res!(delimited!(char!('"'), chain!(char!('#') ~ value: enclosed_content, || { value }), char!('"')),
		|v| { string(v).map(|v| Event::Statement(v)) }));

named!(enclosed_item(&[u8]) -> Event,
	map_res!(delimited!(char!('"'), enclosed_content, char!('"')),
		|v| { string(v).map(|v| Event::Item(v)) }));

#[cfg(test)]
mod tests {
	use nom::IResult::Done;
	use super::Event;

	#[test]
	fn next() {
		assert_eq!(super::next(b"test"), Done(&b""[..], Event::Item("test".into())));
		assert_eq!(super::next(b"\"test\""), Done(&b""[..], Event::Item("test".into())));
		assert_eq!(super::next(b"#test"), Done(&b""[..], Event::Statement("test".into())));
		assert_eq!(super::next(b"\"#test\""), Done(&b""[..], Event::Statement("test".into())));
		assert_eq!(super::next(b"{"), Done(&b""[..], Event::GroupStart));
		assert_eq!(super::next(b"}"), Done(&b""[..], Event::GroupEnd));
	}

	#[test]
	fn bare() {
		assert_eq!(super::bare(b"test"), Done(&b""[..], Event::Item("test".into())));
		assert_eq!(super::bare(b"#test"), Done(&b""[..], Event::Statement("test".into())));

		assert_eq!(super::bare(b"lol wut"), Done(&b" wut"[..], Event::Item("lol".into())));
		assert_eq!(super::bare(b"#lol wut"), Done(&b" wut"[..], Event::Statement("lol".into())));

		assert_eq!(super::bare(b"lol{"), Done(&b"{"[..], Event::Item("lol".into())));
		assert_eq!(super::bare(b"#lol{"), Done(&b"{"[..], Event::Statement("lol".into())));

		assert_eq!(super::bare(b"lol}"), Done(&b"}"[..], Event::Item("lol".into())));
		assert_eq!(super::bare(b"#lol}"), Done(&b"}"[..], Event::Statement("lol".into())));
	}

	#[test]
	fn enclosed() {
		assert_eq!(super::enclosed(b"\"test\""), Done(&b""[..], Event::Item("test".into())));
		assert_eq!(super::enclosed(b"\"#test\""), Done(&b""[..], Event::Statement("test".into())));

		assert_eq!(super::enclosed(b"\"te\\\"st\""), Done(&b""[..], Event::Item("te\"st".into())));
		assert_eq!(super::enclosed(b"\"#te\\\"st\""), Done(&b""[..], Event::Statement("te\"st".into())));
	}

	#[test]
	fn open() {
		assert_eq!(super::open(b"{"), Done(&b""[..], Event::GroupStart));
	}

	#[test]
	fn close() {
		assert_eq!(super::close(b"}"), Done(&b""[..], Event::GroupEnd));
	}
}
