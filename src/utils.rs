#[derive(Clone, Copy)]
pub enum Interface {
	OpenGL
}

pub struct Color {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8
}

impl Color {
	pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
		Self {
			red,
			green,
			blue,
			alpha
		}
	}

	pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
		Color::rgba(red, green, blue, 255)
	}

	pub fn black() -> Self {
		Color::rgb(0, 0, 0)
	}

	pub fn white() -> Self {
		Color::rgb(255, 255, 255)
	}

	pub fn red() -> Self {
		Color::rgb(255, 0, 0)
	}

	pub fn green() -> Self {
		Color::rgb(0, 255, 0)
	}

	pub fn blue() -> Self {
		Color::rgb(0, 0, 255)
	}

	pub fn yellow() -> Self {
		Color::rgb(255, 255, 0)
	}

	pub fn magenta() -> Self {
		Color::rgb(255, 0, 255)
	}

	pub fn cyan() -> Self {
		Color::rgb(0, 255, 255)
	}
    
    pub fn normalize(&self) -> [f32; 4] {
        [
            (self.red / 255).into(),
            (self.green / 255).into(),
            (self.blue / 255).into(),
            (self.alpha / 255).into()
        ]
    }
}
