use std::io::{Read, Seek, SeekFrom};
use std::time::{UNIX_EPOCH, Duration, SystemTime};
use byteorder::{ReadBytesExt, LittleEndian, BigEndian};
use {Result as Res};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Details {
	pub build:    Build,
	pub receiver: Option<Receiver>,
	pub serial:   Serial,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Build {
	pub revision:   i32,
	pub bootloader: SystemTime,
	pub firmware:   SystemTime,
	pub radio:      SystemTime,
}

impl Build {
	pub fn parse<R: Read + Seek>(mut buffer: R) -> Res<Build> {
		let mut revision   = 0;
		let mut bootloader = 0;
		let mut firmware   = 0;
		let mut radio      = 0;

		while let Ok(key) = buffer.read_u8() {
			match key {
				0x09 => {
					revision = try!(buffer.read_i32::<LittleEndian>());
				}

				0x0a => {
					bootloader = try!(buffer.read_i32::<LittleEndian>());
				}

				0x04 => {
					firmware = try!(buffer.read_i32::<LittleEndian>());
				}

				0x05 => {
					radio = try!(buffer.read_i32::<LittleEndian>());
				}

				_ => {
					try!(buffer.seek(SeekFrom::Current(4)));
				}
			}
		}

		Ok(Build {
			revision: revision,

			bootloader: UNIX_EPOCH + Duration::from_secs(bootloader as u64),
			firmware:   UNIX_EPOCH + Duration::from_secs(firmware as u64),
			radio:      UNIX_EPOCH + Duration::from_secs(radio as u64),
		})
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Serial {
	pub mainboard:  [u8; 10],
	pub controller: [u8; 10],
}

impl Serial {
	pub fn parse<R: Read>(mut buffer: R) -> Res<[u8; 10]> {
		try!(buffer.read_u8());

		let mut serial = [0u8; 10];
		try!(buffer.read(&mut serial[..]));

		Ok(serial)
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Receiver {
	pub firmware: SystemTime,
	pub serial:   [u8; 10],
}

impl Receiver {
	pub fn parse<R: Read + Seek>(mut buffer: R) -> Res<Receiver> {
		let firmware = try!(buffer.read_i32::<BigEndian>());
		try!(buffer.seek(SeekFrom::Current(10)));

		let mut serial = [0u8; 10];
		try!(buffer.read(&mut serial[..]));

		Ok(Receiver {
			firmware: UNIX_EPOCH + Duration::from_secs(firmware as u64),
			serial:   serial,
		})
	}
}
