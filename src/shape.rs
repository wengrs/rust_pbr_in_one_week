use crate::vector::Vec3d;
use crate::ray::Ray;

pub trait Shape {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Hit;
}

#[derive(Clone, Debug)]
pub struct Hit {
    pub p: Vec3d,
    pub n: Vec3d,
    pub t: f64,
    pub h: bool,
    pub f: bool,
}

impl Hit {
    pub fn miss() -> Hit {
        Hit{ p:Vec3d::zero(), n:Vec3d::zero(), t:f64::INFINITY, h:false, f:false}
    }
    pub fn set_face(r: &Ray, out_norm: Vec3d) -> bool {
        Vec3d::dot(r.dir, out_norm) < 0.
    }
    pub fn set_norm(f: bool, out_norm: Vec3d) -> Vec3d {
        if f == true {
            return out_norm;
        }
        else {
            return -out_norm;
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Vec3d,
    pub radius: f64,
}

impl Shape for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Hit {
        let oc = r.ori - self.center;
        let a = r.dir.lensq();
        let hb = Vec3d::dot(oc, r.dir);
        let c = oc.lensq() - self.radius*self.radius;
        let dis = hb*hb - a*c;
        if dis < 0. {
            return Hit::miss();
        }
        let sqrtd = dis.sqrt();

        // Find the nearest root.
        let mut root = (-hb - sqrtd) / a;
        if root < tmin || root > tmax {
            root = (-hb + sqrtd) / a;
            if root < tmin || root > tmax {
                return Hit::miss();
            }
        }
        let t = root;
        let p = r.at(t);
        let out_norm = ((p - self.center)/self.radius).norm();
        let h = true;
        let f = Hit::set_face(r, out_norm);
        let n = Hit::set_norm(f, out_norm);
        Hit{t, p, n, h, f}
    }
}

pub fn hit_list(list: &Vec<Box<dyn Shape>>, r: &Ray, tmin: f64, tmax: f64) -> Hit {
    let mut curr_hit = Hit::miss();
    for shape in list {
        let hit = shape.hit(r, tmin, tmax);
        if hit.h == true && hit.t < curr_hit.t {
            curr_hit = hit;
        }
    }
    curr_hit
}