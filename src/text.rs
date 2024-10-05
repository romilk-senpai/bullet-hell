use crate::{
    canvas::Canvas,
    drawable::{DrawParams, Drawable},
};

pub struct Text {}

impl Drawable for Text {
    fn Draw(&self, canvas: &mut Canvas, params: DrawParams) {
        todo!()
    }
}
