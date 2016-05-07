use std::collections::HashMap;
use config::Binding;

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct Preset {
	pub id:       u32,
	pub name:     String,
	pub sources:  HashMap<u32, Source>,
	pub bindings: HashMap<Button, Vec<Binding>>,
}

mod button;
pub use self::button::Button;

mod source;
pub use self::source::Source;
