extern crate steamy_controller as controller;

fn main() {
	let mut manager    = controller::Manager::new().unwrap();
	let mut controller = manager.open().unwrap();

	controller.off().unwrap();
}
