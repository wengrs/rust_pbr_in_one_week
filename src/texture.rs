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

pub struct ImageTexture {
    pub data: bmp::Image,
    pub width: u32,
    pub height: u32,
}

impl ImageTexture {
    pub fn new(filename: &str) -> ImageTexture {
        let path = std::path::Path::new(&filename);
        let data = bmp::open(path).unwrap_or_else(|e| {
            panic!("Failed to open: {}", e);
        });
        let width = data.get_width();
        let height = data.get_height();

        ImageTexture{data, width, height}
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Vec3d) -> RGB {
        let u = clamp(u, 0., 1.);
        let v = 1. - clamp(v, 0., 1.);
        let mut i = (u*self.width as f64) as u32;
        let mut j = (v*self.height as f64) as u32;
        if i >= self.width {i = self.width - 1}
        if j >= self.height{j = self.height - 1}
        let pixel = self.data.get_pixel(i, j);

        RGB::from_pixel(pixel)
    }
}

fn clamp(v: f64, down: f64, up: f64) -> f64
{
    if v > up
    {
        return up;
    }
    else if v < down
    {
        return down;
    }
    else
    {
        return v;
    }
}
