use usb;
use super::Result as Res;
use super::Controller;
use super::{VENDOR_ID, PRODUCT_ID, ENDPOINT, INDEX};

pub struct Manager {
	usb: usb::Context,
}

impl Manager {
	pub fn new() -> Res<Manager> {
		Ok(Manager {
			usb: try!(usb::Context::new()),
		})
	}

	pub fn open(&mut self) -> Res<Controller> {
		let devices = try!(self.usb.devices());

		for mut device in devices.iter() {
			let descriptor = try!(device.device_descriptor());

			if descriptor.vendor_id() != VENDOR_ID {
				continue;
			}

			for (&product, (&endpoint, &index)) in PRODUCT_ID.iter().zip(ENDPOINT.iter().zip(INDEX.iter())) {
				if descriptor.product_id() != product {
					continue;
				}
				
				let handle = try!(device.open());

				return Controller::new(device, handle, product, endpoint, index);
			}
		}

		return Err(usb::Error::NoDevice.into());
	}
}
