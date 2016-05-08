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
pub struct ButtonDiamond<'a> {
	normal: Option<&'a Group>,
	shift:  Option<&'a Group>,

	shifted: bool,
}

impl<'a> ButtonDiamond<'a> {
	pub fn load(normal: Option<&'a Group>, shift: Option<&'a Group>) -> Res<ButtonDiamond<'a>> {
		Ok(ButtonDiamond {
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

impl<'a> Button for ButtonDiamond<'a> {
	fn button(&mut self, device: &mut uinput::Device, _at: Instant, button: input::Button, press: bool) -> Res<HashSet<&Binding>> {
		let bindings = if let Some(bindings) = self.bindings() {
			match bindings {
				&group::Bindings::FourButtons { ref a, ref b, ref x, ref y } => {
					match button {
						input::Button::A => iter(a.iter().flat_map(|b| b.iter())),
						input::Button::B => iter(b.iter().flat_map(|b| b.iter())),
						input::Button::X => iter(x.iter().flat_map(|b| b.iter())),
						input::Button::Y => iter(y.iter().flat_map(|b| b.iter())),
						_                => unreachable!(),
					}
				}

				&group::Bindings::DPad { ref north, ref south, ref east, ref west, .. } => {
					match button {
						input::Button::A => iter(south.iter().flat_map(|b| b.iter())),
						input::Button::B => iter(east.iter().flat_map(|b| b.iter())),
						input::Button::X => iter(west.iter().flat_map(|b| b.iter())),
						input::Button::Y => iter(north.iter().flat_map(|b| b.iter())),
						_                => unreachable!(),
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
