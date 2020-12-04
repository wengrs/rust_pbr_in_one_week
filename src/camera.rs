use crate::vector::Vec3d;
use crate::ray::Ray;

#[derive(Clone, Debug)]
pub struct Camera {
    pub origin: Vec3d,
    pub lower_left_corner: Vec3d,
    pub horizontal: Vec3d,
    pub vertical: Vec3d,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16./9.; 
        let view_height = 2.;
        let view_width = view_height*aspect_ratio;
        let focal_length = 1.;
        
        let origin = Vec3d::zero();
        let horizontal = Vec3d::new(view_width, 0., 0.);
        let vertical = Vec3d::new(0., view_height, 0.);
        let lower_left_corner = origin - horizontal/2. - vertical/2. - Vec3d::new(0., 0., focal_length);

        Camera{origin, horizontal, vertical, lower_left_corner}
    }
    pub fn get_ray(&self, u:f64, v:f64) ->Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}