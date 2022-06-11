use std::ffi::CString;

use winapi::{
	shared::{
		minwindef::PROC,
		windef::{HDC, HGLRC}
	},
	um::libloaderapi::{GetModuleHandleA, GetProcAddress}
};

pub type DeviceContextHandle = HDC;
pub type RenderingContextHandle = HGLRC;
pub type Proc = PROC;

pub fn get_proc_address(proc_name: &str) -> Result<Proc, String> {
	let proc_name = CString::new(proc_name).unwrap();
	let module_name = CString::new("OpenGL32.dll").unwrap();

	let module = unsafe { GetModuleHandleA(module_name.as_ptr()) };

	if module.is_null() {
		return Err("Invalid proc name".to_owned());
	}

	let proc_addr = unsafe { GetProcAddress(module, proc_name.as_ptr()) };

	if proc_addr.is_null() {
		return Err("Invalid proc name".to_owned());
	}

	Ok(proc_addr)
}
