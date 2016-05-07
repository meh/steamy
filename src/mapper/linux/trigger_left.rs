use std::iter;
use std::time::Instant;
use std::collections::HashSet;
use {Result as Res, Error};
use uinput;
use input;
use config::{Group, group};
use util::iter;

#[derive(Debug)]
pub struct TriggerLeft<'a> {
	normal: Option<&'a Group>,
	shift:  Option<&'a Group>,

	shifted: bool,
}

impl<'a> TriggerLeft<'a> {
	pub fn load(normal: Option<&'a Group>, shift: Option<&'a Group>) -> Res<TriggerLeft<'a>> {
		Ok(TriggerLeft {
			normal: normal,
			shift:  shift,

			shifted: false,
		})
	}

	pub fn group(&self) -> Option<&Group> {
		if self.shifted {
			self.shift
		}
		else {
			self.normal
		}
	}

	pub fn bindings(&self) -> Option<&group::Bindings> {
		self.group().map(|g| &g.bindings)
	}

	pub fn button(&self, device: &mut uinput::Device, _at: Instant, button: input::Button, press: bool) -> Res<HashSet<uinput::Event>> {
		let bindings = if let Some(bindings) = self.bindings() {
			match bindings {
				&group::Bindings::Trigger { ref click } => {
					match button {
						input::Button::TriggerLeft => iter(click.iter().flat_map(|b| b.iter())),
						_                          => unreachable!(),
					}
				}

				_ =>
					return Err(Error::NotSupported)
			}
		}
		else {
			iter(iter::empty())
		};

		Ok(bindings.map(|binding|
			device.send(binding, if press { 1 } else { 0 })
				.map(|_| binding.into())).collect()?)
	}
}
