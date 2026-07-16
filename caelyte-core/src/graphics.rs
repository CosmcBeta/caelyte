use std::error::Error;

pub use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::color::Color;

pub trait GraphicsBackend<W>: Sized {
    type InitError: Error;

    fn new(window: W, width: u32, height: u32) -> Result<Self, Self::InitError>
    where
        W: HasWindowHandle + HasDisplayHandle + Clone;

    fn resize(&mut self, width: u32, height: u32);

    fn clear(&mut self, color: Color);

    fn draw_pixel(&mut self, x: i32, y: i32, color: Color);

    fn present(&mut self);
}
