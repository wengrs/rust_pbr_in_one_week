use crate::vector::Vec3d;

#[derive(Clone, Debug)]
pub struct Ray {
    pub ori: Vec3d,
    pub dir: Vec3d,
}

impl Ray {
    pub fn new(ori: Vec3d, dir: Vec3d) -> Ray {
        Ray {ori, dir}
    }
    pub fn at(self, t: f64) -> Vec3d {
        self.ori + t*self.dir
    }
}