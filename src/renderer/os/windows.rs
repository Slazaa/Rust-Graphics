use winapi::{
	shared::windef::{
		HDC,
		HGLRC
	},
	um::wingdi::{
		wglCreateContext,
		wglDeleteContext
	}
};

pub type DeviceContextHandle = HDC;
pub type RenderingContextHandle = HGLRC;

pub fn create_opengl_context(context_handle: DeviceContextHandle) -> RenderingContextHandle {
	unsafe {
		wglCreateContext(context_handle)
	}
}

pub fn delete_opengl_context(context_handle: RenderingContextHandle) {
	unsafe {
		wglDeleteContext(context_handle);
	}
}