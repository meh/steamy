use std::ops::Deref;
use super::{Entry, Parse};

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
	/// Try to convert the value to the given type.
	pub fn to<T: Parse>(&self) -> Option<T> {
		T::parse(&self.0)
	}
}
