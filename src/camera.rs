use crate::vector::Vec3d;
use crate::ray::Ray;

#[derive(Clone, Debug)]
pub struct Camera {
    pub origin: Vec3d,
    pub horizontal: Vec3d,
    pub vertical: Vec3d,
    pub lower_left_corner: Vec3d,
    pub u: Vec3d,
    pub v: Vec3d,
    pub w: Vec3d,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(look_from:Vec3d, look_at:Vec3d, up: Vec3d, vfov: f64, aspect_ratio: f64, aperture: f64, focus_length: f64) -> Camera {
        let theta = vfov*std::f64::consts::PI/180.;
        let h = (theta / 2.).tan();
        let view_height = 2.*h;
        let view_width = view_height*aspect_ratio;
        
        let w = (look_from - look_at).norm();
        let u = Vec3d::cross(up, w).norm();
        let v = Vec3d::cross(w, u);

        let origin = look_from;
        let horizontal = focus_length*view_width*u;
        let vertical = focus_length*view_height*v;
        let lower_left_corner = origin - horizontal/2. - vertical/2. - focus_length*w;
        let lens_radius = aperture / 2.;
        Camera{origin, horizontal, vertical, lower_left_corner, u, v, w, lens_radius}
    }
    pub fn get_ray(&self, s:f64, t:f64) ->Ray {
        let rd = self.lens_radius*Vec3d::rand_in_unit_disk();
        let offset = self.u*rd.x + self.v*rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset)
    }
}