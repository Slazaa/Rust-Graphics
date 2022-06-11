use std::ffi::CString;

use winapi::um::wingdi::{
	SwapBuffers,
	wglCreateContext,
	wglDeleteContext,
	wglGetProcAddress,
	wglMakeCurrent
};

use crate::renderer::{
	DeviceContextHandle,
	RenderingContextHandle,
	Proc
};

pub fn create_context(device_context: DeviceContextHandle) -> RenderingContextHandle {
	unsafe { wglCreateContext(device_context) }
}

pub fn make_current(device_context: DeviceContextHandle, rendering_context: RenderingContextHandle) {
	unsafe { wglMakeCurrent(device_context, rendering_context); }
}

pub fn delete_context(rendering_context: RenderingContextHandle) {
	unsafe { wglDeleteContext(rendering_context); }
}

pub fn get_proc_address(proc_name: &str) -> Result<Proc, String> {
	unsafe {
		let proc_name = CString::new(proc_name).unwrap();
		let proc_addr = wglGetProcAddress(proc_name.as_ptr());

		if proc_addr.is_null() {
			return Err("Invalid proc name".to_owned());
		}

		Ok(proc_addr)
	}
}

pub fn display(device_context: DeviceContextHandle) {
	unsafe { SwapBuffers(device_context); }
}