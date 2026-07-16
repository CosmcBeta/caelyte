use caelyte_core::graphics::GraphicsBackend;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct Engine<B: GraphicsBackend<Arc<Window>>> {
    window: Option<Arc<Window>>,
    pub renderer: Option<B>,
}

impl<B: GraphicsBackend<Arc<Window>>> Engine<B> {
    pub fn new() -> Self {
        Self {
            window: None,
            renderer: None,
        }
    }
}

impl<B: GraphicsBackend<Arc<Window>>> ApplicationHandler for Engine<B> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap(),
            );

            let size = window.inner_size();
            let renderer = B::new(window.clone(), size.width, size.height);

            self.window = Some(window);
            self.renderer = Some(renderer.unwrap());
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.present();
                }
            }
            _ => (),
        }
    }
}
