use crate::error::CaelyteError;

#[derive(Debug, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    pub const YELLOW: Self = Self::rgb(255, 255, 0);
    pub const PURPLE: Self = Self::rgb(255, 0, 255);
    pub const CYAN: Self = Self::rgb(0, 255, 255);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn hsl() {
        todo!()
    }

    pub fn hsla() {
        todo!()
    }

    pub fn hwb() {
        todo!()
    }

    pub fn lch() {
        todo!()
    }

    pub fn hex(&self, hex: &str) -> Result<Self, CaelyteError> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);

        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).map_err(|_| "invalid hex character")?;
                let g = u8::from_str_radix(&hex[1..2], 16).map_err(|_| "invalid hex character")?;
                let b = u8::from_str_radix(&hex[2..3], 16).map_err(|_| "invalid hex character")?;
                Ok(Self { r, g, b, a: 255 })
            }
            4 => {
                let r = u8::from_str_radix(&hex[0..1], 16).map_err(|_| "invalid hex character")?;
                let g = u8::from_str_radix(&hex[1..2], 16).map_err(|_| "invalid hex character")?;
                let b = u8::from_str_radix(&hex[2..3], 16).map_err(|_| "invalid hex character")?;
                let a = u8::from_str_radix(&hex[3..4], 16).map_err(|_| "invalid hex character")?;
                Ok(Self { r, g, b, a })
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "invalid hex character")?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "invalid hex character")?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "invalid hex character")?;
                Ok(Self { r, g, b, a: 255 })
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "invalid hex character")?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "invalid hex character")?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "invalid hex character")?;
                let a = u8::from_str_radix(&hex[6..8], 16).map_err(|_| "invalid hex character")?;
                Ok(Self { r, g, b, a })
            }
            _ => Err("hex string is wrong length".into()),
        }
    }

    pub fn to_u32(&self) -> u32 {
        (self.r as u32) << 24 | (self.g as u32) << 16 | (self.b as u32) << 8 | (self.a as u32)
    }
}
