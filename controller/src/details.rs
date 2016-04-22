use std::io::{Read, Seek, SeekFrom};
use std::time::{UNIX_EPOCH, Duration, SystemTime};
use byteorder::{ReadBytesExt, LittleEndian};
use {Result as Res};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Details {
	pub firmware: SystemTime,
}

impl Details {
	pub fn parse<R: Read + Seek>(mut buffer: R) -> Res<Details> {
		try!(buffer.seek(SeekFrom::Current(23)));
		let firmware = try!(buffer.read_i32::<LittleEndian>());
		try!(buffer.seek(SeekFrom::Current(37)));

		Ok(Details {
			firmware: UNIX_EPOCH + Duration::from_secs(firmware as u64),
		})
	}
}
