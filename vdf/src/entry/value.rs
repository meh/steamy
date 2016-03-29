use std::ops::Deref;
use super::Entry;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Value(String);

impl From<String> for Value {
	fn from(value: String) -> Value {
		Value(value)
	}
}

impl Into<Entry> for Value {
	fn into(self) -> Entry {
		Entry::Value(self)
	}
}

impl Deref for Value {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Value {
	pub fn to<T: Parse>(&self) -> Option<T> {
		T::parse(&self.0)
	}
}

pub trait Parse: Sized {
	fn parse(string: &str) -> Option<Self>;
}

macro_rules! from_str {
	(for) => ();

	(for $ty:ident $($rest:tt)*) => (
		from_str!($ty);
		from_str!(for $($rest)*);
	);

	($ty:ident) => (
		impl Parse for $ty {
			fn parse(string: &str) -> Option<Self> {
				string.parse::<$ty>().ok()
			}
		}
	);
}

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
from_str!(for IpAddr Ipv4Addr Ipv6Addr SocketAddr SocketAddrV4 SocketAddrV6);
from_str!(for i8 i16 i32 i64 isize u8 u16 u32 u64 usize f32 f64);

impl Parse for bool {
	fn parse(string: &str) -> Option<Self> {
		match string {
			"0" => Some(false),
			"1" => Some(true),
			v   => v.parse::<bool>().ok()
		}
	}
}
