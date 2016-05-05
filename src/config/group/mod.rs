#[derive(Clone, Debug)]
pub struct Group {
	pub id:       u32,
	pub mode:     Mode,
	pub bindings: Bindings,
	pub settings: Settings,
	pub actions:  Actions,
}

mod mode;
pub use self::mode::Mode;

mod bindings;
pub use self::bindings::Bindings;

mod settings;
pub use self::settings::Settings;

mod actions;
pub use self::actions::Actions;
