use crate::color::RGB;
use crate::vector::Vec3d;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3d) -> RGB;
}

pub struct SolidTexture {
    pub color: RGB,
}

impl Texture for SolidTexture {
    fn value(&self, _u: f64, _v: f64, _p: Vec3d) -> RGB {
        self.color
    }
}
