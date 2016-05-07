macro_rules! wait {
	($body:expr, $time:expr) => (
		if let Ok(value) = $body {
			value
		}
		else {
			thread::sleep(Duration::from_millis($time));
			continue;
		}
	);
}

macro_rules! end {
	($body:expr) => (
		if let Ok(value) = $body {
			value
		}
		else {
			break;
		}
	);
}

pub fn iter<'a, T: Iterator + 'a>(it: T) -> Box<Iterator<Item=T::Item> + 'a> {
	Box::new(it)
}
