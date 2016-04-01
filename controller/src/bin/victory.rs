extern crate steamy_controller as controller;
use controller::sound::Note;

use std::time::Duration;
use std::thread;

macro_rules! play {
	($controller:expr, ?, $ms:expr) => {
		::std::thread::sleep(Duration::from_millis($ms));
	};

	($controller:expr, $note:ident, $ms:expr) => {
		$controller.sound().left().note(Note::$note).duration(Duration::from_millis($ms)).play().unwrap();
		$controller.sound().right().note(Note::$note).duration(Duration::from_millis($ms)).play().unwrap();
		::std::thread::sleep(Duration::from_millis($ms));
	};

	($controller:expr, $note:ident #, $ms:expr) => {
		$controller.sound().left().note(Note::$note).sharp().duration(Duration::from_millis($ms)).play().unwrap();
		$controller.sound().right().note(Note::$note).sharp().duration(Duration::from_millis($ms)).play().unwrap();
		::std::thread::sleep(Duration::from_millis($ms));
	};

	($controller:expr, $note:ident, $oct:expr, $ms:expr) => {
		$controller.sound().left().note(Note::$note).octave($oct).duration(Duration::from_millis($ms)).play().unwrap();
		$controller.sound().right().note(Note::$note).octave($oct).duration(Duration::from_millis($ms)).play().unwrap();
		::std::thread::sleep(Duration::from_millis($ms));
	};

	($controller:expr, $note:ident #, $oct:expr, $ms:expr) => {
		$controller.sound().left().note(Note::$note).sharp().octave($oct).duration(Duration::from_millis($ms)).play().unwrap();
		$controller.sound().right().note(Note::$note).sharp().octave($oct).duration(Duration::from_millis($ms)).play().unwrap();
		::std::thread::sleep(Duration::from_millis($ms));
	};
}

fn main() {
	let mut manager    = controller::Manager::new().unwrap();
	let mut controller = manager.open().unwrap();

	play!(controller, C, 7, 200);
	play!(controller, C, 7, 200);
	play!(controller, C, 7, 200);
	play!(controller, C, 7, 500);

	play!(controller, G, 6, 500);
	play!(controller, A#, 6, 500);

	play!(controller, C, 7, 300);
	play!(controller, A#, 6, 200);
	play!(controller, C, 7, 500);
}
