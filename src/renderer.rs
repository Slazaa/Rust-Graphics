use winapi::{
	shared::windef::HGLRC, 
	um::wingdi::wglCreateContext
};

pub struct Renderer {
	context: HGLRC
}

impl Renderer {
	pub fn new(device_context: DeviceContext) -> Self {
		Self {
			context: unsafe { wglCreateContext(window_handle) }
		}
	}
}