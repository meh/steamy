use std::time::Duration;
use std::io::{self, Cursor};
use byteorder::{WriteBytesExt};
use usb;

use super::Result as Res;
use super::{State, Feedback, Sensors};

/// The controller.
pub struct Controller<'a> {
	device: usb::Device<'a>,
	handle: usb::DeviceHandle<'a>,

	product:  u16,
	endpoint: u8,
	address:  u8,
	index:    u16,
}

impl<'a> Controller<'a> {
	#[doc(hidden)]
	pub fn new<'b>(mut device: usb::Device<'b>, mut handle: usb::DeviceHandle<'b>, product: u16, endpoint: u8, index: u16) -> Res<Controller<'b>> {
		let mut address: Option<u8> = None;

		for i in 0 .. try!(device.device_descriptor()).num_configurations() {
			for interface in try!(device.config_descriptor(i)).interfaces() {
				if try!(handle.kernel_driver_active(interface.number())) {
					try!(handle.detach_kernel_driver(interface.number()));
				}

				for descriptor in interface.descriptors() {
					if descriptor.class_code() == 3 &&
					   descriptor.sub_class_code() == 0 &&
					   descriptor.protocol_code() == 0
					{
						try!(handle.claim_interface(descriptor.interface_number()));
					}

					for end in descriptor.endpoint_descriptors() {
						if end.number() == endpoint {
							address = Some(end.address());
						}
					}
				}
			}
		}

		let mut controller = Controller {
			device: device,
			handle: handle,

			product:  product,
			endpoint: endpoint,
			address:  try!(address.ok_or(usb::Error::InvalidParam)),
			index:    index,
		};

		try!(controller.reset());

		return Ok(controller);
	}

	fn reset(&mut self) -> Res<()> {
		try!(self.sensors().off());
		try!(self.control(Duration::from_secs(0), |mut buf| {
			try!(buf.write_u8(0x81));

			Ok(())
		}));

		Ok(())
	}

	/// Send a raw USB control message to the controller.
	pub fn control<T, F: FnOnce(Cursor<&mut [u8]>) -> io::Result<T>>(&mut self, timeout: Duration, func: F) -> Res<()> {
		let mut buf = [0u8; 64];

		try!(func(Cursor::new(&mut buf)));
		try!(self.handle.write_control(0x21, 0x09, 0x0300, self.index, &buf, timeout));

		Ok(())
	}

	/// Get the feedback builder.
	pub fn feedback<'b>(&'b mut self) -> Feedback<'b, 'a> where 'a: 'b {
		Feedback::new(self)
	}

	/// Get the sensor manager.
	pub fn sensors<'b>(&'b mut self) -> Sensors<'b, 'a> where 'a: 'b {
		Sensors::new(self)
	}

	/// Turn the controller off.
	pub fn off(&mut self) -> Res<()> {
		try!(self.control(Duration::from_secs(0), |mut buf| {
			try!(buf.write_u8(0x9f));
			try!(buf.write_u8(0x04));
			try!(buf.write_u8(0x6f));
			try!(buf.write_u8(0x66));
			try!(buf.write_u8(0x66));
			try!(buf.write_u8(0x21));

			Ok(())
		}));

		Ok(())
	}

	/// Get the current state of the controller.
	pub fn state(&mut self, timeout: Duration) -> Res<State> {
		let mut buf = [0u8; 64];

		if try!(self.handle.read_interrupt(self.address, &mut buf, timeout)) != buf.len() {
			return Err(usb::Error::NotSupported.into());
		}

		match try!(State::parse(Cursor::new(&buf[..]))) {
			state@State::Power(true) => {
				try!(self.reset());

				Ok(state)
			}

			state => {
				Ok(state)
			}
		}
	}
}
