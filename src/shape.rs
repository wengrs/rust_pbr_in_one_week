use crate::vector::Vec3d;
use crate::ray::Ray;
use crate::material::Material;
use crate::material;
use crate::aabb::AABB;
use std::sync::Arc;

pub trait Shape{
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Hit;
    fn bound(&self, t0: f64, t1: f64) -> AABB;
}

#[derive(Clone)]
pub struct Hit {
    pub p: Vec3d,
    pub n: Vec3d,
    pub t: f64,
    pub h: bool,
    pub u: f64,
    pub v: f64,
    pub f: bool,
    pub mat: Arc<dyn Material>,
}

impl Hit {
    pub fn miss() -> Hit {
        Hit{ p:Vec3d::zero(), n:Vec3d::zero(), t:f64::INFINITY, h:false, u:0., v:0., f:false, mat:Arc::new(material::Nothing{})}
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

pub struct Objects {
    pub object: Vec<Box<dyn Shape>>,
}

impl Objects {
    pub fn new(object: Vec<Box<dyn Shape>>) -> Objects {
        Objects{object}
    }
}

impl Shape for Objects {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Hit {
        if self.bound(tmin, tmax).hit(r, tmin, tmax) == false {
            return Hit::miss();
        }
        let mut curr_hit = Hit::miss();
        for shape in &self.object {
            let hit = shape.hit(r, tmin, tmax);
            if hit.h == true && hit.t < curr_hit.t {
                curr_hit = hit;
            }
        }
        curr_hit           
    }
    fn bound(&self, t0: f64, t1: f64) -> AABB {
        if self.object.len() == 0 {
            return AABB::zero();
        }
        let mut first_box = true;
        let mut temp_box = AABB::zero();
        for o in &self.object {
            temp_box = if first_box == true{o.bound(t0, t1)} else {AABB::union_box(&o.bound(t0, t1), &temp_box)};
            first_box = false;
        }
        temp_box
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3d,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl Shape for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Hit {
        if self.bound(tmin, tmax).hit(r, tmin, tmax) == false {
            return Hit::miss();
        }        
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
        let n = Hit::set_norm(f, out_norm).norm();
        let (u, v) = self.get_sphere_uv(out_norm);
        Hit{t, p, n, h, u, v, f, mat:Arc::clone(&self.mat)}
    }
    fn bound(&self, _: f64, _: f64) -> AABB {
        let p1 = self.center - self.radius*Vec3d::one();
        let p2 = self.center + self.radius*Vec3d::one();
        AABB::new(p1, p2)
    }
}

impl Sphere {
    pub fn get_sphere_uv(&self, p: Vec3d) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + std::f64::consts::PI;
        let u = phi/(2.*std::f64::consts::PI);
        let v = theta/std::f64::consts::PI;
        (u, v)
    }
}

#[derive(Clone)]
pub struct MovingSphere {
    pub c0: Vec3d,
    pub c1: Vec3d,
    pub t0: f64,
    pub t1: f64,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn center(&self, t:f64) -> Vec3d {
        self.c0 + (t-self.t0)/(self.t1-self.t0)*(self.c1-self.c0)
    }
}

impl Shape for MovingSphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Hit {
        let oc = r.ori - self.center(r.t);
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
        let out_norm = ((p - self.center(r.t))/self.radius).norm();
        let h = true;
        let f = Hit::set_face(r, out_norm);
        let n = Hit::set_norm(f, out_norm).norm();
        let (u, v) = self.get_sphere_uv(out_norm);
        Hit{t, p, n, h, u, v, f, mat:Arc::clone(&self.mat)}
    }
    fn bound(&self, t0: f64, t1: f64) -> AABB {
        let p00 = self.center(t0) - self.radius*Vec3d::one();
        let p01 = self.center(t0) + self.radius*Vec3d::one();
        let p10 = self.center(t1) - self.radius*Vec3d::one();
        let p11 = self.center(t1) + self.radius*Vec3d::one();
        AABB::union_box(&AABB::new(p00, p01), &AABB::new(p10, p11))
    }
}

impl MovingSphere {
    pub fn get_sphere_uv(&self, p: Vec3d) -> (f64, f64) {
        let theta = -p.y.acos();
        let phi = -p.z.atan2(p.x) + std::f64::consts::PI;
        let u = phi/(2.*std::f64::consts::PI);
        let v = theta/std::f64::consts::PI;
        (u, v)
    }
}