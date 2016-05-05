use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct Preset {
	pub id:      u32,
	pub name:    String,
	pub sources: HashMap<u32, Source>,
}

mod source;
pub use self::source::Source;
