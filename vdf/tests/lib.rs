extern crate steamy_vdf as vdf;

#[test]
fn loader() {
	let config = vdf::load("tests/desktop.vdf").unwrap();

	assert_eq!(2.0,
		config.lookup("controller_mappings.version").unwrap()
		.to::<f32>().unwrap());

	assert_eq!("four_buttons",
		config.lookup("controller_mappings.group.0.mode").unwrap()
		.as_str().unwrap());

	assert_eq!(false,
		config.lookup("controller_mappings.group.1.settings.requires_click").unwrap()
		.to::<bool>().unwrap());
}
