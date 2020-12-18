use crate::color::RGB;
use crate::vector::Vec3d;

pub trait Texture{
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

pub struct CheckerTexture {
    pub odd:Box<dyn Texture>,
    pub even:Box<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3d) -> RGB {
        let sines = (10.*p.x).sin()*(10.*p.y).sin()*(10.*p.z).sin();
        if sines < 0. {
            return self.odd.value(u, v, p);
        }
        else {
            return self.even.value(u, v, p);
        }
    }
}

impl CheckerTexture {
    pub fn new(odd: RGB, even: RGB) -> CheckerTexture {
        let odd = Box::new(SolidTexture{color: odd});
        let even = Box::new(SolidTexture{color: even});
        CheckerTexture{odd, even}
    }
}