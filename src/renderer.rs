mod os;

use crate::utils::{Interface, Color};

#[cfg(unix)]
use os::unix::*;

#[cfg(windows)]
use os::windows::*;

pub struct Renderer {
	device_context: DeviceContextHandle,
    rendering_context: RenderingContextHandle,
	interface: Interface
}

impl Renderer {
	pub fn new(device_context: DeviceContextHandle, interface: Interface) -> Self {
        let rendering_context = create_context(device_context, interface);
	
        unsafe {
            if winapi::um::wingdi::wglGetCurrentContext().is_null() {
                panic!("No current context");
            }

            gl::load_with(|s| get_proc_address(s).unwrap() as *const _);
        }

        Self {
			device_context,
            rendering_context,
			interface
		}
	}

	pub fn clear(&mut self, color: Color) {
        unsafe {
            match self.interface {
                Interface::OpenGL => {
                    let color = color.normalize();
                    gl::ClearColor(color[0], color[1], color[2], color[3]);
                }
            }
        }
	}

	pub fn display(&mut self) {
		
	}
}

impl Drop for Renderer {
    fn drop(&mut self) {
        delete_context(self.rendering_context, self.interface);
    }
}
