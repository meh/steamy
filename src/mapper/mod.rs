use {Result as Res, Config};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::Mapper;

pub fn new(config: Config) -> Res<Mapper> {
	Mapper::new(config)
}
