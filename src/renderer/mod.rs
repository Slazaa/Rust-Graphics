pub mod unix;
pub mod windows;

#[cfg(unix)]
use unix::*;

#[cfg(windows)]
use windows::*;

use crate::utils::Color;

pub struct Renderer {
    window_handle: WindowHandle,
	device_context: DeviceContextHandle
}

impl Renderer {
	pub fn new(window_handle: WindowHandle) -> Result<Self, String> {
        Ok(Self {
            window_handle,
            device_context: match create_context(window_handle) {
                Ok(x) => x,
                Err(e) => return Err(e)
            }
        })
	}

	pub fn clear(&mut self, color: Color) {
		todo!();
	}

	pub fn display(&mut self) {
        todo!();
	}
}

impl Drop for Renderer {
	fn drop(&mut self) {
        release_context(self.window_handle, self.device_context);
	}
}
