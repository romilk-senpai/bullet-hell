use std::ptr::null_mut;
use std::thread;
use std::time::{Duration, Instant};

use crate::engine::Engine;
use crate::graphics::window::Window;

pub trait Handler {
    fn update(&self);
	fn draw(&self);
}

pub struct Runner{
    engine: Engine,
    window: Window,
    handler: Box<dyn Handler>,
    running: bool,
}

impl Runner {
    pub fn new(engine: Engine, handler: Box<dyn Handler>) -> Self {
        Self {
            engine,
            window: Window::new(),
            handler,
            running: false,
        }
    }

    pub fn run(&mut self, eng: &Engine) {
		self.running = true;
        let frame_duration = Duration::from_millis(100);
        let mut last_frame_time = Instant::now();

		while self.running
        {
            let current_time = Instant::now();
            let elapsed_time = current_time - last_frame_time;

            if elapsed_time >= frame_duration {
                self.handler.update();
                self.handler.draw();

                last_frame_time = current_time;
            }

            thread::sleep(Duration::from_millis(1));
        }
    }

    pub fn stop(&mut self, eng: &Engine) {
		self.running = false;
	}
}
