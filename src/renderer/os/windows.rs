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

pub fn create_opengl_context(context_handle: DeviceContextHandle) -> Result<RenderingContextHandle, String> {
	unsafe {
		let context = wglCreateContext(context_handle);

		if context.is_null() {
			return Err("Failed to create OpenGL context".to_owned());
		} 

		Ok(context)
	}
}

pub fn delete_opengl_context(context_handle: RenderingContextHandle) {
	unsafe {
		wglDeleteContext(context_handle);
	}
}