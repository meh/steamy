use std::iter;
use std::collections::HashSet;
use {Result as Res, Error};
use uinput;
use input;
use config::group;
use super::util::iter;

pub fn button(device: &mut uinput::Device, bindings: &group::Bindings, button: input::Button, press: bool) -> Res<HashSet<uinput::Event>> {
	let bindings = match bindings {
		&group::Bindings::FourButtons { ref a, ref b, ref x, ref y } => {
			match button {
				input::Button::Down  => iter(a.iter().flat_map(|b| b.iter())),
				input::Button::Right => iter(b.iter().flat_map(|b| b.iter())),
				input::Button::Left  => iter(x.iter().flat_map(|b| b.iter())),
				input::Button::Up    => iter(y.iter().flat_map(|b| b.iter())),
				input::Button::Pad   => iter(iter::empty()),
				_                    => unreachable!(),
			}
		}

		&group::Bindings::DPad { ref north, ref south, ref east, ref west, ref click } => {
			match button {
				input::Button::Down  => iter(south.iter().flat_map(|b| b.iter())),
				input::Button::Right => iter(east.iter().flat_map(|b| b.iter())),
				input::Button::Left  => iter(west.iter().flat_map(|b| b.iter())),
				input::Button::Up    => iter(north.iter().flat_map(|b| b.iter())),
				input::Button::Pad   => iter(click.iter().flat_map(|b| b.iter())),
				_                    => unreachable!(),
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
