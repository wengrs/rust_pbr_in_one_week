use crate::vector::Vec3d;
use crate::ray::Ray;
use crate::color::RGB;
use crate::shape::Hit;

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
    fn scatter(&self, _: &Ray, _: &Hit) -> Scatter {
        let s = false;
        let r = Ray::new(Vec3d::zero(), Vec3d::zero());
        let a = RGB::black();
        Scatter{s, r, a}
    }
}

pub struct Lambertian {
    pub albedo: RGB,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Scatter {
        let s = true;
        let mut dir = hit.n + Vec3d::rand_in_unit_sphere().norm();
        if dir.near_zero() {
            dir = hit.n;
        }
        let r = Ray::new(hit.p, dir);
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
        let r = Ray::new(hit.p, reflected + self.fuzz*Vec3d::rand_in_unit_sphere());
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
        let rf = Vec3d::refract(r_in.dir.norm(), hit.n, rf_ratio);
        let r = Ray::new(hit.p, rf);
        Scatter{s:true, r, a}
    }
}