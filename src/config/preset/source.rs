use {Result as Res};
use config::Input;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Source {
	pub id:     u32,
	pub input:  Input,
	pub active: bool,
	pub shift:  bool,
}

impl Source {
	pub fn load(id: u32, string: &str) -> Res<Self> {
		let matches = string.split(' ').collect::<Vec<&str>>();
		let input   = Input::parse(ok!(matches.get(0))?)?;
		let active  = ok!(matches.get(1).map(|s| *s))? == "active";
		let shift   = matches.get(2).map(|s| *s).unwrap_or("") == "modeshift";

		Ok(Source {
			id:     id,
			input:  input,
			active: active,
			shift:  shift,
		})
	}
}
