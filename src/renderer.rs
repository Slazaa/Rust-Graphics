mod os;

use crate::utils::Color;

#[cfg(unix)]
pub type DeviceContextHandle = os::unix::DeviceContextHandle;
#[cfg(unix)]
pub type RenderingContextHandle = os::unix::RenderingContextHandle;

#[cfg(windows)]
pub type DeviceContextHandle = os::windows::DeviceContextHandle;
#[cfg(windows)]
pub type RenderingContextHandle = os::windows::RenderingContextHandle;

pub enum Interface {
	OpenGL
}

pub struct Renderer {
	interface: Interface,
	context: RenderingContextHandle
}

impl Renderer {
	pub fn new(context_handle: DeviceContextHandle, interface: Interface) -> Result<Self, String> {
		let context = match interface {
			Interface::OpenGL => create_opengl_context(context_handle)
		};

		let context = match context {
			Ok(x) => x,
			Err(e) => return Err(e)
		};

		Ok(Self {
			interface,
			context 
		})
	}

	pub fn clear(&mut self, color: Color) {
		
	}

	pub fn display(&mut self) {

	}
}

impl Drop for Renderer {
	fn drop(&mut self) {
		#[cfg(unix)]
		match self.interface {
			Interface::OpenGL => os::unix::delete_opengl_context(self.context)
		}

		#[cfg(windows)]
		match self.interface {
			Interface::OpenGL => os::windows::delete_opengl_context(self.context)
		}
	}
}

fn create_opengl_context(context_handle: DeviceContextHandle) -> Result<RenderingContextHandle, String> {
	#[cfg(unix)]
	return os::unix::create_opengl_context(context_handle);

	#[cfg(windows)]
	return os::windows::create_opengl_context(context_handle);
}