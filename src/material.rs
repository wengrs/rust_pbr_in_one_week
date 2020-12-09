use crate::vector::Vec3d;
use crate::ray::Ray;
use crate::color::RGB;
use crate::shape::Hit;
extern crate rand;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Scatter {
    pub s: bool,
    pub r: Ray,
    pub a: RGB,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Scatter;
}

pub struct Nothing {
}

impl Material for Nothing {
    fn scatter(&self, r_in: &Ray, _: &Hit) -> Scatter {
        let s = false;
        let r = Ray::new(Vec3d::zero(), Vec3d::zero(), r_in.t);
        let a = RGB::black();
        Scatter{s, r, a}
    }
}

pub struct Lambertian {
    pub albedo: RGB,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Scatter {
        let s = true;
        let mut dir = hit.n + Vec3d::rand_in_unit_sphere().norm();
        if dir.near_zero() {
            dir = hit.n;
        }
        let r = Ray::new(hit.p, dir, r_in.t);
        let a = self.albedo;
        Scatter{s, r, a}
    }    
}

pub struct Metal {
    pub albedo: RGB,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Scatter {
        let reflected = Vec3d::reflect(r_in.dir, hit.n);
        let s = Vec3d::dot(reflected, hit.n) > 0.;
        let r = Ray::new(hit.p, reflected + self.fuzz*Vec3d::rand_in_unit_sphere(), r_in.t);
        let a = self.albedo;
        Scatter{s, r, a}
    }  
}

pub struct Dielectric {
    pub ir: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Scatter {
        let a = Vec3d::one().to_rgb();
        let rf_ratio = if hit.f {1./self.ir} else {self.ir};
        let cos_theta = f64::min(Vec3d::dot(-r_in.dir.norm(), hit.n.norm()), 1.);
        let sin_theta = (1. - cos_theta*cos_theta).sqrt();
        let not_refract  = rf_ratio * sin_theta > 1.;
        let dir: Vec3d;
        if not_refract || Dielectric::reflectance(cos_theta, rf_ratio) > rand::thread_rng().gen_range(0., 1.){
            dir = Vec3d::reflect(r_in.dir, hit.n);
        }
        else {
            dir = Vec3d::refract(r_in.dir, hit.n, rf_ratio);
        }
        let r = Ray::new(hit.p, dir, r_in.t);
        Scatter{s:true, r, a}
    }
}

impl Dielectric {
    pub fn reflectance(cosine:f64, ref_idx:f64) -> f64 {
        // Schlock's approximation
        let r0 = (1.-ref_idx)/(1.+ref_idx);
        let r0 = r0 * r0;
        r0 + (1.-r0)*((1.-cosine).powi(5))
    }
}