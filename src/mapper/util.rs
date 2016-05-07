macro_rules! source {
	($config:expr, $id:expr, $input:expr, $active:expr, $shift:expr) => (
		$config.presets.get(&$id).unwrap().sources.values()
			.find(|s|
				s.input  == $input &&
				s.active == $active &&
				s.shift  == $shift)
			.map(|s|
				s.id)
	);
}

macro_rules! group_by {
	($config:expr, $id:expr, $input:expr, $active:expr, $shift:expr) => (
		source!($config, $id, $input, $active, $shift)
			.and_then(|id| $config.groups.get(&id))
	);
}
