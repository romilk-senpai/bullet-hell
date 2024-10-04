use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

use super::GraphicsContext;

pub struct WgpuContext {}

impl GraphicsContext for WgpuContext {
    fn create_window() -> super::window::Window
    where
        Self: Sized,
    {
        let event_loop = EventLoop::new().unwrap();
        #[allow(unused_mut)]
        let mut builder = winit::window::WindowBuilder::new();
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowBuilderExtWebSys;
            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            builder = builder.with_canvas(Some(canvas));
        }
        let window = builder.build(&event_loop).unwrap();

        #[cfg(not(target_arch = "wasm32"))]
        {
            env_logger::init();
            pollster::block_on(run(event_loop, window));
        }
        #[cfg(target_arch = "wasm32")]
        {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init().expect("could not initialize logger");
            wasm_bindgen_futures::spawn_local(run(event_loop, window));
        }

        super::window::Window::new()
    }
}
