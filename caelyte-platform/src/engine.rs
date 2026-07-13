use std::sync::Arc;
use caelyte_core::graphics::GraphicsBackend;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct Engine<B: GraphicsBackend> {
    window: Option<Arc<Window>>,
    pub renderer: Option<B>,
}

impl<B: GraphicsBackend> Engine<B> {
    pub fn new() -> Self {
        Self {
            window: None,
            renderer: None,
        }
    }
}

impl<B: GraphicsBackend> ApplicationHandler for Engine<B> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let renderer = B::new(window.clone());

        self.window = Some(window);
        self.renderer = Some(renderer);
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
