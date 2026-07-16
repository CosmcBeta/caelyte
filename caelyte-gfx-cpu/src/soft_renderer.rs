use std::num::NonZeroU32;

use caelyte_core::{
    color::Color,
    graphics::{GraphicsBackend, HasDisplayHandle, HasWindowHandle},
};
use softbuffer::{Context, Surface};

use crate::error::CaelyteError;

pub struct SoftRenderer<W> {
    _context: Context<W>,
    surface: Surface<W, W>,
    buffer: Vec<u32>,
    width: u32,
    height: u32,
}

impl<W> GraphicsBackend<W> for SoftRenderer<W>
where
    W: HasWindowHandle + HasDisplayHandle + Clone,
{
    type InitError = CaelyteError;

    fn new(window: W, width: u32, height: u32) -> Result<Self, Self::InitError> {
        let context = Context::new(window.clone())?;
        let surface = Surface::new(&context, window.clone())?;

        let buffer = vec![0; (width * height) as usize];

        Ok(Self {
            _context: context,
            surface,
            buffer,
            width,
            height,
        })
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.width = width;
        self.height = height;

        self.buffer.resize((width * height) as usize, 0);

        if let (Some(w), Some(h)) = (NonZeroU32::new(width), NonZeroU32::new(height)) {
            self.surface.resize(w, h).unwrap();
        }
    }

    fn clear(&mut self, color: Color) {
        let hex_color = color.to_u32();

        self.buffer.fill(hex_color);
    }

    fn draw_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }

        let index = (y as u32 * self.width + x as u32) as usize;

        self.buffer[index] = color.to_u32();
    }

    fn present(&mut self) {
        let mut window_buffer = self.surface.buffer_mut().unwrap();

        window_buffer.copy_from_slice(&self.buffer);

        window_buffer.present().unwrap();
    }
}
