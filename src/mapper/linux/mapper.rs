use std::collections::HashSet;
use uinput;
use {Result as Res};
use config::{self, Config, group, Binding};
use input::{self, Event};
use super::util::iter;
use super::{button_diamond, trackpad_left, trigger_left, trigger_right};

pub struct Mapper {
	config:  Config,
	device:  uinput::Device,
	preset:  u32,
	pressed: HashSet<uinput::Event>,
}

impl Mapper {
	pub fn new(config: Config) -> Res<Self> {
		let builder = uinput::default()?.name("steamy")?;

		// Enable events from modes.
		let builder = config.groups.iter()
			.map(|(_, group)|
				group.mode)
			.fold(builder, |builder, mode|
				match mode {
					group::Mode::JoystickMove | group::Mode::MouseJoystick =>
						builder.event(uinput::event::absolute::Position::X).unwrap().min(-32768).max(32767).fuzz(16).flat(128)
						       .event(uinput::event::absolute::Position::Y).unwrap().min(-32768).max(32767).fuzz(16).flat(128),

					group::Mode::AbsoluteMouse =>
						builder.event(uinput::event::relative::Position::X).unwrap()
						       .event(uinput::event::relative::Position::Y).unwrap(),

					_ =>
						builder
				});

		// Enable events from bindings.
		let builder = config.groups.iter()
			.flat_map(|(_, group)|
				match group.bindings {
					group::Bindings::FourButtons { ref a, ref b, ref x, ref y } =>
						iter(a.iter().chain(b.iter()).chain(x.iter()).chain(y.iter())),

					group::Bindings::DPad { ref north, ref south, ref east, ref west, ref click } =>
						iter(north.iter().chain(south.iter()).chain(east.iter()).chain(west.iter()).chain(click.iter())),

					group::Bindings::AbsoluteMouse { ref click, ref double } =>
						iter(click.iter().chain(double.iter())),

					group::Bindings::Trigger { ref click } =>
						iter(click.iter()),

					group::Bindings::ScrollWheel { ref cw, ref ccw, ref click } =>
						iter(cw.iter().chain(ccw.iter()).chain(click.iter())),

					group::Bindings::MouseJoystick { ref click } =>
						iter(click.iter()),

					group::Bindings::JoystickMove { ref click } =>
						iter(click.iter()),

					group::Bindings::TouchMenu { ref buttons } =>
						iter(buttons.iter().flat_map(|v| v.iter()))
				})
			.flat_map(|binding|
				binding.iter())
			.filter(|&binding|
				if let &Binding::Action(..) = binding {
					false
				}
				else {
					true
				})
			.fold(builder, |builder, binding|
				builder.event(binding).unwrap());

		Ok(Mapper {
			config:  config,
			device:  builder.create()?,
			preset:  0,
			pressed: HashSet::new(),
		})
	}

	pub fn send(&mut self, event: Event) -> Res<()> {
		match event {
			Event::Connected => (),
			Event::Disconnected => {
				for event in self.pressed.drain() {
					self.device.send(event, 0)?;
				}
			}

			Event::Button(btn@input::Button::A, press) |
			Event::Button(btn@input::Button::B, press) |
			Event::Button(btn@input::Button::X, press) |
			Event::Button(btn@input::Button::Y, press) => {
				if let Some(bindings) = bindings!(self, config::Input::ButtonDiamond, true, false) {
					button!(self, button_diamond, bindings, btn, press);
				}
			}

			Event::Button(btn@input::Button::Up, press) |
			Event::Button(btn@input::Button::Down, press) |
			Event::Button(btn@input::Button::Left, press) |
			Event::Button(btn@input::Button::Right, press) |
			Event::Button(btn@input::Button::Pad, press) => {
				if let Some(bindings) = bindings!(self, config::Input::TrackpadLeft, true, false) {
					button!(self, trackpad_left, bindings, btn, press);
				}
			}

			Event::Button(btn@input::Button::TriggerLeft, press) => {
				if let Some(bindings) = bindings!(self, config::Input::TriggerLeft, true, false) {
					button!(self, trigger_left, bindings, btn, press);
				}
			}

			Event::Button(btn@input::Button::TriggerRight, press) => {
				if let Some(bindings) = bindings!(self, config::Input::TriggerRight, true, false) {
					button!(self, trigger_right, bindings, btn, press);
				}
			}

			_ => ()
		}

		self.device.synchronize()?;

		Ok(())
	}
}
