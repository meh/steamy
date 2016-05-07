use {Result as Res, Config};

#[macro_use]
mod util;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::Mapper;

pub fn new(config: &Config) -> Res<Mapper> {
	Mapper::new(config)
}
