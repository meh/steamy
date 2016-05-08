use std::time::Instant;
use std::collections::HashSet;
use uinput;
use {Result as Res};
use input;
use config::Binding;

pub trait Button {
	fn button(&mut self, device: &mut uinput::Device, at: Instant, button: input::Button, press: bool) -> Res<HashSet<&Binding>>;
}

pub trait Shift {
	fn shift(&mut self);
}
