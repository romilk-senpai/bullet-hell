use cgmath::Point2;

use crate::graphics::drawable::DrawableQuad;

use super::buffer::QuadBufferBuilder;

pub struct Image {
    position: Point2<f32>,
    size: Point2<f32>,
    pivot: Point2<f32>,
}

impl Image {
    pub fn new(position: Point2<f32>, size: Point2<f32>, pivot: Point2<f32>) -> Image {
        Self {
            position,
            size,
            pivot,
        }
    }

    fn Draw(&mut self, buffer: &mut QuadBufferBuilder) {
        let min_x = self.position.x - self.size.x * self.pivot.x;
        let min_y = self.position.y - self.size.y * self.pivot.y;
        let max_x = self.position.x + self.size.x * self.pivot.x;
        let max_y = self.position.y + self.size.y * self.pivot.y;

        buffer.push_quad(min_x, min_y, max_x, max_y);
    }
}

impl DrawableQuad for Image {
    
}
