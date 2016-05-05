use std::collections::HashMap;
use vdf;
use super::{Config, Group, Preset};
use super::group::{self, Mode};
use super::preset;
use {Result as Res, Error};

pub fn load(table: &vdf::Entry) -> Res<Config> {
	let title       = lookup!(table@Title as str)?.into();
	let description = lookup!(table@Description as str)?.into();
	let creator     = lookup!(table@Creator as str)?.into();

	let mut groups = HashMap::new();
	for group in lookup!(table@group as slice)? {
		let id   = lookup!(group@ID as u32)?;
		let mode = lookup!(group@Mode as Mode)?;

		let bindings = if let Ok(entry) = lookup!(group@bindings) {
			group::Bindings::load(mode, entry)?
		}
		else {
			group::Bindings::empty(mode)
		};

		let settings = if let Ok(entry) = lookup!(group@settings) {
			group::Settings::load(mode, entry)?
		}
		else {
			group::Settings::default()
		};

		let actions = if let Ok(entry) = lookup!(group@actions) {
			group::Actions::load(mode, entry)?
		}
		else {
			group::Actions::default()
		};

		groups.insert(id, Group {
			id:       id,
			mode:     mode,
			bindings: bindings,
			settings: settings,
			actions:  actions,
		});
	}

	let mut presets = HashMap::new();
	for preset in lookup!(table@preset as slice)? {
		let id   = lookup!(preset@ID as u32)?;
		let name = lookup!(preset@Name as str)?.into();

		let sources = lookup!(preset@group_source_bindings as table)?.iter().map(|(key, entry)| {
			let id     = key.parse::<u32>().map_err(|_| Error::InvalidParameter)?;
			let source = preset::Source::load(id, ok!(entry.as_str())?)?;

			Ok((id, source))
		}).collect::<Res<HashMap<u32, preset::Source>>>()?;

		let switch = 

		presets.insert(id, Preset {
			id:      id,
			name:    name,
			sources: sources,
		});
	}

	Ok(Config {
		title:       title,
		description: description,
		creator:     creator,

		groups:  groups,
		presets: presets,
	})
}
