use super::buffer::QuadBufferBuilder;

pub trait DrawableQuad {
    fn Draw(&self, buffer: &mut QuadBufferBuilder);
}
