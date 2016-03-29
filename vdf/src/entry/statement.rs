use std::ops::Deref;
use super::Entry;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Statement(String);

impl From<String> for Statement {
	fn from(value: String) -> Self {
		Statement(value)
	}
}

impl Into<Entry> for Statement {
	fn into(self) -> Entry {
		Entry::Statement(self)
	}
}

impl Deref for Statement {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
