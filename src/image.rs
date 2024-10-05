use crate::{
    canvas::Canvas,
    drawable::{DrawParams, Drawable},
};

pub struct Image {}

impl Drawable for Image {
    fn Draw(&self, canvas: &mut Canvas, params: DrawParams) {
        todo!()
    }
}
