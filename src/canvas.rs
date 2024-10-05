use crate::drawable::{DrawParams, Drawable};

pub struct Canvas {}

impl Canvas {
    pub fn Draw(&mut self, drawable: &impl Drawable, params: DrawParams) {
        drawable.Draw(self, params);
    }
}
