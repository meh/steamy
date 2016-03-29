#[cfg(not(target_os = "linux"))]
use std::marker::PhantomData;

use std::time::Duration;
use std::io::{self, Cursor};
use byteorder::{WriteBytesExt};

#[cfg(target_os = "linux")]
use usb;

#[cfg(not(target_os = "linux"))]
use hid;

use {Result as Res, Error, State, Feedback, Sensors};

/// The controller.
#[cfg(target_os = "linux")]
pub struct Controller<'a> {
	handle: usb::DeviceHandle<'a>,

	address: u8,
	index:   u16,
}

#[cfg(not(target_os = "linux"))]
pub struct Controller<'a> {
	handle: hid::Handle,
	marker: PhantomData<&'a ()>,
}

impl<'a> Controller<'a> {
	#[doc(hidden)]
	#[cfg(target_os = "linux")]
	pub fn new<'b>(mut device: usb::Device<'b>, mut handle: usb::DeviceHandle<'b>, endpoint: u8, index: u16) -> Res<Controller<'b>> {
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
			handle:  handle,
			address: try!(address.ok_or(usb::Error::InvalidParam)),
			index:   index,
		};

		try!(controller.reset());

		Ok(controller)
	}

	fn reset(&mut self) -> Res<()> {
		try!(self.sensors().off());
		try!(self.control(|mut buf| {
			try!(buf.write_u8(0x81));

			Ok(())
		}));

		Ok(())
	}

	#[cfg(not(target_os = "linux"))]
	pub fn new<'b>(handle: hid::Handle) -> Res<Controller<'b>> {
		let mut controller = Controller {
			handle: handle,
			marker: PhantomData,
		};

		try!(controller.reset());

		Ok(controller)
	}

	#[doc(hidden)]
	#[cfg(target_os = "linux")]
	pub fn control<T, F: FnOnce(Cursor<&mut [u8]>) -> io::Result<T>>(&mut self, func: F) -> Res<()> {
		let mut buf = [0u8; 64];

		try!(func(Cursor::new(&mut buf)));
		try!(self.handle.write_control(0x21, 0x09, 0x0300, self.index, &buf, Duration::from_secs(0)));

		Ok(())
	}

	#[cfg(not(target_os = "linux"))]
	pub fn control<T, F: FnOnce(Cursor<&mut [u8]>) -> io::Result<T>>(&mut self, func: F) -> Res<()> {
		let mut buf = [0u8; 65];
		buf[0] = 0x00;

		try!(func(Cursor::new(&mut buf[1..])));
		try!(self.handle.feature().send_direct(&buf[..]));

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
		try!(self.control(|mut buf| {
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
	#[cfg(target_os = "linux")]
	pub fn state(&mut self, timeout: Duration) -> Res<State> {
		let mut buf = [0u8; 64];

		if try!(self.handle.read_interrupt(self.address, &mut buf, timeout)) != buf.len() {
			return Err(Error::InvalidParameter);
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

	#[cfg(not(target_os = "linux"))]
	pub fn state(&mut self, timeout: Duration) -> Res<State> {
		let mut buf = [0u8; 65];

		if try!(self.handle.data().read_direct(&mut buf[..], timeout)).unwrap_or(0) != buf.len() - 1 {
			return Err(Error::InvalidParameter);
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
