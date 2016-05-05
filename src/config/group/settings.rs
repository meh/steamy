use vdf;
use config::group::Mode;
use {Result as Res};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Settings {
	pub haptic:   Haptic,
	pub scroll:   Scroll,
	pub button:   Button,
	pub pad:      Pad,
	pub menu:     Menu,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Haptic {
	pub intensity: Option<u32>,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Scroll {
	pub angle: Option<u32>,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Button {
	pub size:     Option<u32>,
	pub dist:     Option<u32>,
	pub required: bool,
	pub repeat:   Option<u32>,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Pad {
	pub deadzone:    Option<u32>,
	pub tap:         Option<u32>,
	pub sensitivity: Option<u32>,
	pub friction:    Option<u32>,
	pub smoothing:   bool,
	pub edge:        Edge,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Edge {
	pub binding: Binding,
	pub spin:    Spin,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Binding {
	pub radius: Option<u32>,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Spin {
	pub velocity: Option<u32>,
	pub radius:   Option<u32>,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Menu {
	pub count:    Option<usize>,
	pub opacity:  Option<u8>,
	pub position: Position,
	pub scale:    Option<u8>,
	pub labels:   bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Position {
	pub x: Option<u32>,
	pub y: Option<u32>,
}

impl Settings {
	pub fn load(_mode: Mode, table: &vdf::Entry) -> Res<Self> {
		let mut settings = Settings::default();

		settings.haptic.intensity        = lookup!(table@haptic_intensity as u32).ok();
		settings.scroll.angle            = lookup!(table@scroll_angle as u32).ok();
		settings.button.size             = lookup!(table@button_size as u32).ok();
		settings.button.dist             = lookup!(table@button_dist as u32).ok();
		settings.button.required         = lookup!(table@requires_click as bool).unwrap_or(false);
		settings.button.repeat           = lookup!(table@hold_repeat_interval as u32).ok();
		settings.pad.deadzone            = lookup!(table@deadzone as u32).ok();
		settings.pad.tap                 = lookup!(table@doubetap_max_duration as u32).ok();
		settings.pad.sensitivity         = lookup!(table@sensitivity as u32).ok();
		settings.pad.smoothing           = lookup!(table@mouse_smoothing as bool).unwrap_or(false);
		settings.pad.edge.binding.radius = lookup!(table@edge_binding_radius as u32).ok();
		settings.pad.edge.spin.velocity  = lookup!(table@edge_spin_velocity as u32).ok();
		settings.pad.edge.spin.radius    = lookup!(table@edge_spin_radius as u32).ok();
		settings.menu.count              = lookup!(table@touch_menu_button_count as usize).ok();
		settings.menu.opacity            = lookup!(table@touch_menu_opacity as u8).ok();
		settings.menu.position.x         = lookup!(table@touch_menu_position_x as u32).ok();
		settings.menu.position.y         = lookup!(table@touch_menu_position_y as u32).ok();
		settings.menu.scale              = lookup!(table@touch_menu_scale as u8).ok();
		settings.menu.labels             = lookup!(table@touch_menu_show_labels as bool).unwrap_or(false);

		Ok(settings)
	}
}
