#[cfg(target_os = "linux")]
use usb;

#[cfg(target_os = "windows")]
use hid;

use {Result as Res, Controller};
use {VENDOR_ID, PRODUCT_ID, ENDPOINT, INDEX};

/// Controller manager.
pub struct Manager {
	#[cfg(target_os = "linux")]
	usb: usb::Context,

	#[cfg(target_os = "windows")]
	hid: hid::Manager,
}

impl Manager {
	/// Create a new controller manager.
	#[cfg(target_os = "linux")]
	pub fn new() -> Res<Manager> {
		Ok(Manager {
			usb: try!(usb::Context::new()),
		})
	}

	#[cfg(target_os = "windows")]
	pub fn new() -> Res<Manager> {
		Ok(Manager {
			hid: try!(hid::init()),
		})
	}

	/// Open a controller.
	#[cfg(target_os = "linux")]
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

				return Controller::new(device, handle, endpoint, index);
			}
		}

		return Err(usb::Error::NoDevice.into());
	}

	#[cfg(target_os = "windows")]
	pub fn open(&self) -> Res<Controller> {
		for &product in &PRODUCT_ID {
			for device in self.hid.find(Some(VENDOR_ID), Some(product)) {
				if let Ok(handle) = device.open() {
					return Controller::new(handle);
				}
			}
		}

		return Err(hid::Error::NotFound.into());
	}
}
