macro_rules! ok {
	($body:expr) => (
		$body.ok_or($crate::Error::InvalidParameter)
	);
}

macro_rules! lookup {
	($table:ident @ $($path:tt).+) => (
		ok!($table.lookup(stringify!($($path).*))).or_else(|_|
			ok!($table.lookup(&stringify!($($path).*).to_lowercase())))
	);

	($table:ident @ $($path:tt).+ as table) => (
		lookup!($table @ $($path).*)
			.and_then(|v| ok!(v.as_table()))
	);

	($table:ident @ $($path:tt).+ as slice) => (
		lookup!($table @ $($path).*)
			.and_then(|v| ok!(v.as_slice()))
	);

	($table:ident @ $($path:tt).+ as statement) => (
		lookup!($table @ $($path).*)
			.and_then(|v| ok!(v.as_statement()))
	);

	($table:ident @ $($path:tt).+ as value) => (
		lookup!($table @ $($path).*)
			.and_then(|v| ok!(v.as_value()))
	);

	($table:ident @ $($path:tt).+ as str) => (
		lookup!($table @ $($path).*)
			.and_then(|v| ok!(v.as_str()))
	);

	($table:ident @ $($path:tt).+ as $ty:ty) => (
		lookup!($table @ $($path).*)
			.and_then(|v| ok!(v.to::<$ty>()))
	);
}

macro_rules! binding {
	($table:ident @ $($path:tt).+) => (
		binding!($table, stringify!($($path).*))
			.and_then(|v| if v.is_none() {
				binding!($table, stringify!($($path).*).to_lowercase())
			}
			else {
				Ok(v)
			})
	);

	($table:ident, $path:expr) => (
		if let Some(value) = $table.lookup($path).and_then(|v| v.as_slice()) {
			value.iter()
				.map($crate::config::Binding::load)
				.collect::<$crate::Result<Vec<$crate::config::Binding>>>()
				.map(|v| Some(v))
		}
		else {
			Ok(None)
		}
	);
}
