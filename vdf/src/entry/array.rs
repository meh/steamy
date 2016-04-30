use std::ops::{Deref, DerefMut};
use super::Entry;

/// An array of entries (items that have the same key).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Array(Vec<Entry>);

impl From<Entry> for Array {
	fn from(value: Entry) -> Self {
		Array(vec![value])
	}
}

impl Into<Entry> for Array {
	fn into(self) -> Entry {
		Entry::Array(self)
	}
}

impl Deref for Array {
	type Target = Vec<Entry>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Array {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
