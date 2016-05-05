use std::path::Path;
use std::collections::HashMap;
use vdf;
use {Result as Res, Error};

#[macro_use]
mod util;

mod input;
pub use self::input::Input;

pub mod binding;
pub use self::binding::Binding;

pub mod group;
pub use self::group::Group;

pub mod preset;
pub use self::preset::Preset;

#[derive(Clone, Debug)]
pub struct Config {
	pub title:       String,
	pub description: String,
	pub creator:     String,

	pub groups:  HashMap<u32, Group>,
	pub presets: HashMap<u32, Preset>,
}

pub fn load<P: AsRef<Path>>(path: P) -> Res<Config> {
	let table    = vdf::load(path)?;
	let mappings = lookup!(table@controller_mappings)?;

	match lookup!(mappings@version as u32).unwrap_or(1) {
		1 =>
			v1::load(mappings),

		2 =>
			v2::load(mappings),

		_ =>
			Err(Error::NotSupported),
	}
}

mod v1;
mod v2;
