#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Entry {
	Table(Table),
	Array(Array),
	Statement(Statement),
	Value(Value),
}

impl Entry {
	pub fn lookup<S: AsRef<str>>(&self, path: S) -> Option<&Entry> {
		let mut current = self;

		for name in path.as_ref().split('.') {
			if let Some(entry) = current.get(name) {
				current = entry;
			}
			else {
				return None;
			}
		}

		Some(current)
	}

	pub fn get<S: AsRef<str>>(&self, name: S) -> Option<&Entry> {
		match self {
			&Entry::Table(ref value) =>
				value.get(name.as_ref()),

			&Entry::Array(ref value) =>
				name.as_ref().parse::<usize>().ok().and_then(|i| value.get(i)),

			_ =>
				None
		}
	}

	pub fn to<T: value::Parse>(&self) -> Option<T> {
		if let &Entry::Value(ref value) = self {
			value.to::<T>()
		}
		else {
			None
		}
	}

	pub fn as_table(&self) -> Option<&Table> {
		if let &Entry::Table(ref value) = self {
			Some(value)
		}
		else {
			None
		}
	}

	pub fn as_slice(&self) -> Option<&[Entry]> {
		if let &Entry::Array(ref value) = self {
			Some(value.as_slice())
		}
		else {
			None
		}
	}

	pub fn as_statement(&self) -> Option<&Statement> {
		if let &Entry::Statement(ref value) = self {
			Some(value)
		}
		else {
			None
		}
	}

	pub fn as_value(&self) -> Option<&Value> {
		if let &Entry::Value(ref value) = self {
			Some(value)
		}
		else {
			None
		}
	}

	pub fn as_str(&self) -> Option<&str> {
		match self {
			&Entry::Value(ref value) =>
				Some(&*value),

			&Entry::Statement(ref value) =>
				Some(&*value),

			_ =>
				None
		}
	}
}

mod table;
pub use self::table::Table;

mod array;
pub use self::array::Array;

mod statement;
pub use self::statement::Statement;

mod value;
pub use self::value::Value;
