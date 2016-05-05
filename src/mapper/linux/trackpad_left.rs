use std::iter;
use {Result as Res, Error};
use uinput;
use input::{self, Event};
use config::group;
use super::util::iter;

pub fn button(device: &mut uinput::Device, bindings: &group::Bindings, button: input::Button, press: bool) -> Res<()> {
	match bindings {
		&group::Bindings::FourButtons { ref a, ref b, ref x, ref y } => {
			for binding in match button {
				input::Button::Down  => iter(a.iter().flat_map(|b| b.iter())),
				input::Button::Right => iter(b.iter().flat_map(|b| b.iter())),
				input::Button::Left  => iter(x.iter().flat_map(|b| b.iter())),
				input::Button::Up    => iter(y.iter().flat_map(|b| b.iter())),
				_                    => iter(iter::empty()),
			} {
				device.send(binding, if press { 1 } else { 0 })?;
			}
		}

		&group::Bindings::DPad { ref north, ref south, ref east, ref west, ref click } => {
			for binding in match button {
				input::Button::Down  => iter(south.iter().flat_map(|b| b.iter())),
				input::Button::Right => iter(east.iter().flat_map(|b| b.iter())),
				input::Button::Left  => iter(west.iter().flat_map(|b| b.iter())),
				input::Button::Up    => iter(north.iter().flat_map(|b| b.iter())),
				input::Button::Pad   => iter(click.iter().flat_map(|b| b.iter())),
				_                    => iter(iter::empty()),
			} {
				device.send(binding, if press { 1 } else { 0 })?;
			}
		}

		_ =>
			return Err(Error::NotSupported)
	}

	Ok(())
}
