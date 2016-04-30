use std::ops::Deref;
use std::io::{self, Read};
use std::collections::HashMap;
use reader::{Reader, Event, Item};
use super::{Entry, Statement, Value, Array};

/// A table of entries.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Table(HashMap<String, Entry>);

fn insert(map: &mut HashMap<String, Entry>, key: String, value: Entry) {
	if !map.contains_key(&key) {
		map.insert(key, value);
		return;
	}

	if let Some(&mut Entry::Array(ref mut array)) = map.get_mut(&key) {
		array.push(value);
		return;
	}

	let mut array = Array::from(map.remove(&key).unwrap());
	array.push(value);

	map.insert(key, array.into());
}

impl Table {
	/// Load a table from the given `Reader`.
	pub fn load<R: Read>(reader: &mut Reader<R>) -> io::Result<Table> {
		let mut map = HashMap::new();

		loop {
			match try!(reader.event()) {
				Event::Entry(Item::Statement(..), _) =>
					(),

				Event::Entry(Item::Value(key), Item::Statement(value)) =>
					insert(&mut map, key, Statement::from(value).into()),

				Event::Entry(Item::Value(key), Item::Value(value)) =>
					insert(&mut map, key, Value::from(value).into()),

				Event::GroupStart(name) =>
					insert(&mut map, name, try!(Table::load(reader)).into()),

				Event::GroupEnd | Event::End =>
					break
			}
		}

		return Ok(Table(map));
	}
}

impl Into<Entry> for Table {
	fn into(self) -> Entry {
		Entry::Table(self)
	}
}

impl Deref for Table {
	type Target = HashMap<String, Entry>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
