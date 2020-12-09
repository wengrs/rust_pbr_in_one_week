use crate::vector::Vec3d;

#[derive(Clone, Debug)]
pub struct Ray {
    pub ori: Vec3d,
    pub dir: Vec3d,
    pub t: f64,
}

impl Ray {
    pub fn new(ori: Vec3d, dir: Vec3d, t: f64) -> Ray {
        Ray {ori, dir, t}
    }
    pub fn at(&self, t: f64) -> Vec3d {
        self.ori + t*self.dir
    }
}