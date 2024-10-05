use crate::canvas::Canvas;

pub struct DrawParams {
    position: mint::Point2<f32>,
}

pub trait Drawable {
    fn Draw(&self, canvas: &mut Canvas, params: DrawParams);
}
