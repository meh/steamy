Steam config file parser
========================
Rust library to parse and process VDF files.

Example
=======

```vdf
"controller_mappings"
{
	"version"		"2"
	"group"
	{
		"mode"		"four_buttons"
	}
	"group"
	{
		"settings"
		{
			"requires_click"		"0"
		}
	}
}
```

```rust
extern crate steamy_vdf as vdf;

fn main() {
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
```
