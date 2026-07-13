use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::sync::Arc;

pub trait GraphicsBackend {
    fn new(window: Arc<impl HasWindowHandle + HasDisplayHandle>) -> Self;

    fn resize(&mut self, width: u32, height: u32);

    fn clear(&mut self, r: u8, g: u8, b: u8);

    fn present(&mut self);
}
