pub mod audio;
pub mod graphics;

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Fullscreen, Window, WindowBuilder},
};

use crate::graphics::{renderer::RendererConfig, Renderer};

struct UserEvent;

pub struct Context<'a> {
    pub window: Window,
    pub renderer: Renderer<'a>,
    event_loop: EventLoop<()>,
}

impl<'a> Context<'a> {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
    pub fn new() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            } else {
                env_logger::init();
            }
        }

        let event_loop = EventLoop::new().unwrap();
        let monitor = event_loop.primary_monitor().unwrap();
        let video_mode = monitor.video_modes().next();
        let size = video_mode
            .clone()
            .map_or(PhysicalSize::new(800, 600), |vm| vm.size());
        let window = WindowBuilder::new()
            .with_visible(false)
            .with_title("Pong")
            .with_fullscreen(video_mode.map(|vm| Fullscreen::Exclusive(vm)))
            .build(&event_loop)
            .unwrap();

        window.set_cursor_visible(false);

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas()?);
                    dst.append_child(&canvas).ok()?;

                    // Request fullscreen, if denied, continue as normal
                    match canvas.request_fullscreen() {
                        Ok(_) => {}
                        Err(_) => (),
                    }

                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        log::info!("Setup...");

        let config = RendererConfig::new(size);
        let renderer = pollster::block_on(Renderer::new(&window, config));

        Self {
            window,
            renderer,
            event_loop,
        }
    }

    pub fn run(self) {
        self.event_loop.run(move |event, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    //state.game_state = state::GameState::Quiting;
                }
                _ => {}
            }
        });
    }
}
