use std::ffi::CString;

use core::ptr::null_mut;

use winapi::{
    um::wingdi::{
        wglCreateContext,
        wglDeleteContext,
        wglGetProcAddress,
        wglMakeCurrent
    },
    shared::{
        minwindef::PROC,
        windef::{
            HDC,
            HGLRC
        }
    }
};

use crate::utils::Interface;

pub type DeviceContextHandle = HDC;
pub type RenderingContextHandle = HGLRC;
pub type Proc = PROC;

pub fn create_context(device_context: DeviceContextHandle, interface: Interface) -> RenderingContextHandle {
    unsafe {
        match interface {
            Interface::OpenGL => {
                let rendering_context = wglCreateContext(device_context);
                wglMakeCurrent(device_context, rendering_context);
                rendering_context
            }
        }
    }
}

pub fn delete_context(rendering_context: RenderingContextHandle, interface: Interface) {
    unsafe {
        match interface {
            Interface::OpenGL => {
                wglMakeCurrent(null_mut(), null_mut());
                wglDeleteContext(rendering_context);
            }
        }
    }
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
