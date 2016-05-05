macro_rules! button {
	($events:expr, $current:expr, $new:expr, { }) =>
		();

	($events:expr, $current:expr, $new:expr, { $flag:expr => $button:expr, $($rest:tt)* }) => (
		button!($events, $current, $button, $new, $flag);
		button!($events, $current, $new, { $($rest)* });
	);

	($events:expr, $current:expr, $button:expr, $new:expr, $flag:expr) => (
		if !$current.contains($flag) && $new.contains($flag) {
			$events.push(Event::Button($button, true));
		}

		if $current.contains($flag) && !$new.contains($flag) {
			$events.push(Event::Button($button, false));
		}
	);
}
