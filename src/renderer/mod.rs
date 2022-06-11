pub mod opengl;
pub mod unix;
pub mod windows;

#[cfg(unix)]
use unix::*;

#[cfg(windows)]
use windows::*;

use crate::utils::{Interface, Color};

pub struct Renderer {
	device_context: DeviceContextHandle,
	rendering_context: RenderingContextHandle,
	interface: Interface,
	current: bool,
}

impl Renderer {
	pub fn new(device_context: DeviceContextHandle, interface: Interface) -> Self {
		let mut renderer = Self {
			device_context,
			rendering_context: match interface {
				Interface::OpenGL => opengl::create_context(device_context)
			},
			interface,
			current: false
		};

		renderer.make_current();

		//gl::load_with(|s| get_proc_address(s).unwrap() as *const _);

		renderer
	}

	pub fn make_current(&mut self) {
		opengl::make_current(self.device_context, self.rendering_context);
		self.current = true;
	}

	pub fn is_current(&self) -> bool {
		self.current
	}

	pub fn clear(&mut self, color: Color) {
		todo!();
	}

	pub fn display(&mut self) {
		match self.interface {
			Interface::OpenGL => opengl::display(self.device_context)
		}
	}
}

impl Drop for Renderer {
	fn drop(&mut self) {
		match self.interface {
			Interface::OpenGL => opengl::delete_context(self.rendering_context)
		}
	}
}
