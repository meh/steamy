use std::collections::HashSet;
use uinput;
use {Result as Res, Error};
use input;
use config::group;
use super::util::iter;

pub fn button(device: &mut uinput::Device, bindings: &group::Bindings, button: input::Button, press: bool) -> Res<HashSet<uinput::Event>> {
	let bindings = match bindings {
		&group::Bindings::Trigger { ref click } => {
			match button {
				input::Button::TriggerRight => iter(click.iter().flat_map(|b| b.iter())),
				_                           => unreachable!(),
			}
		}

		_ =>
			return Err(Error::NotSupported)
	};

	let mut events = HashSet::new();

	for binding in bindings {
		device.send(binding, if press { 1 } else { 0 })?;
		events.insert(binding.into());
	}

	Ok(events)
}
