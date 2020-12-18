use crate::vector::Vec3d;
extern crate bmp;

#[derive(Clone, Copy, Debug)]
pub struct RGB
{
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl RGB
{
    pub fn new(r: f64, g: f64, b: f64) -> RGB
    {
        RGB { r: clamp(r, 0., 1.), g: clamp(g, 0., 1.), b: clamp(b, 0., 1.)}
    }
    pub fn black() -> RGB
    {
        RGB::new(0., 0., 0.)
    }
    pub fn white() -> RGB
    {
        RGB::new(1., 1., 1.)
    }
    pub fn red() -> RGB
    {
        RGB::new(1., 0., 0.)
    }
    pub fn green() -> RGB
    {
        RGB::new(0., 1., 0.)
    }
    pub fn blue() -> RGB
    {
        RGB::new(0., 0., 1.)
    }
    pub fn to_vec(&self) -> Vec3d {
        Vec3d::new(self.r, self.g, self.b)
    }
    pub fn pixel(self) ->   bmp::Pixel {
        let r = (self.r * 255.99) as u8;
        let g = (self.g * 255.99) as u8;
        let b = (self.b * 255.99) as u8;
        bmp::Pixel::new(r, g, b)
    }
    pub fn from_pixel(pixel: bmp::Pixel) -> RGB {
        let r = pixel.r as f64 / 255.;
        let g = pixel.g as f64 / 255.;
        let b = pixel.b as f64 / 255.;
        RGB::new(r, g, b)
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
