use vdf;
use {Result as Res, Error};
use super::Config;

pub fn load(_table: &vdf::Entry) -> Res<Config> {
	Err(Error::NotSupported)
}
