use crate::graphics::window::Window;

pub trait GraphicsContext {
    fn create_window() -> Window where Self: Sized;
}