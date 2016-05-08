use std::iter;
use std::time::Instant;
use std::collections::HashSet;
use uinput;
use {Result as Res, Error};
use input;
use config::{Binding, Group, group};
use util::iter;
use super::{util, Button};

#[derive(Debug)]
pub struct PadLeft<'a> {
	normal: Option<&'a Group>,
	shift:  Option<&'a Group>,

	shifted: bool,
}

impl<'a> PadLeft<'a> {
	pub fn load(normal: Option<&'a Group>, shift: Option<&'a Group>) -> Res<PadLeft<'a>> {
		Ok(PadLeft {
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
}

impl<'a> Button for PadLeft<'a> {
	fn button(&mut self, device: &mut uinput::Device, _at: Instant, button: input::Button, press: bool) -> Res<HashSet<&Binding>> {
		let bindings = if let Some(bindings) = self.bindings() {
			match bindings {
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
			}
		}
		else {
			iter(iter::empty())
		};

		util::button(device, bindings, press)
	}
}
