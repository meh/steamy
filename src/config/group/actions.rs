use vdf;
use config::group::Mode;
use {Result as Res};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Actions {

}

impl Actions {
	pub fn load(_mode: Mode, _table: &vdf::Entry) -> Res<Self> {
		Ok(Actions { })
	}
}
