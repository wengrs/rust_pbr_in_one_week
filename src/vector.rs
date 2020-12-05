use std::ops;
use crate::color::RGB;
extern crate rand;
use rand::Rng;

#[derive(Copy,Clone,Debug)]
pub struct Vec3d
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d
{
    pub fn zero() -> Vec3d
    {
        Vec3d{ x: 0., y: 0., z: 0. }
    }
    pub fn one() -> Vec3d
    {
        Vec3d{ x: 1., y: 1., z: 1. }
    }
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d
    {
        Vec3d{ x, y, z }
    }
    pub fn dot(v1: Vec3d, v2: Vec3d) -> f64
    {
        v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
    }
    pub fn cross(v1: Vec3d, v2: Vec3d) -> Vec3d
    {
        let x = v1.y*v2.z - v1.z*v2.y;
        let y = v1.z*v2.x - v1.x*v2.z;
        let z = v1.x*v2.y - v1.y*v2.x;
        Vec3d{ x, y, z }
    }
    pub fn abs(self) -> Vec3d
    {
        let x = f64::abs(self.x);
        let y = f64::abs(self.y);
        let z = f64::abs(self.z);
        Vec3d{ x, y, z }
    }
    pub fn lensq(self) -> f64
    {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn len(self) -> f64
    {
        f64::sqrt(self.lensq())
    }
    pub fn norm(self) -> Vec3d
    {
        self/self.len()
    }
    pub fn min_comp(self) -> f64
    {
        f64::min(self.x, f64::min(self.y, self.z))
    }
    pub fn max_comp(self) -> f64
    {
        f64::max(self.x, f64::max(self.y, self.z))
    }
    pub fn max_dim(self) -> usize
    {
        if self.x > self.y && self.x > self.z
        {
            return 0;
        }
        else if self.y > self.x && self.y > self.z
        {
            return 1;
        }
        else
        {
            return 2;
        }
    }
    pub fn comp_min(v1: Vec3d, v2: Vec3d) -> Vec3d
    {
        let x = f64::min(v1.x, v2.x);
        let y = f64::min(v1.y, v2.y);
        let z = f64::min(v1.z, v2.z);
        Vec3d{ x, y, z}
    }
    pub fn comp_max(v1: Vec3d, v2: Vec3d) -> Vec3d
    {
        let x = f64::max(v1.x, v2.x);
        let y = f64::max(v1.y, v2.y);
        let z = f64::max(v1.z, v2.z);
        Vec3d{ x, y, z}
    }
    pub fn permute(self, i0: usize, i1: usize, i2: usize) -> Vec3d
    {
        Vec3d::new(self[i0], self[i1], self[i2])
    }
    pub fn sqrt(self) -> Vec3d {
        let x = self.x.sqrt();
        let y = self.y.sqrt();
        let z = self.z.sqrt();
        Vec3d{x, y, z}
    }
    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
    pub fn reflect(v: Vec3d, n: Vec3d) -> Vec3d {
        let v = v.norm();
        let n = n.norm();
        v - 2.*Vec3d::dot(v, n)*n
    }
    pub fn refract(v: Vec3d, n: Vec3d, eta_ratio: f64) -> Vec3d {
        let v = v.norm();
        let n = n.norm();
        let cos_theta = f64::min(Vec3d::dot(-v, n), 1.);
        let r_perp = eta_ratio*(v + cos_theta*n);
        let r_para = -(1.-r_perp.lensq()).abs().sqrt()*n;
        r_perp + r_para
    }
    pub fn to_rgb(self) -> RGB {
        let r = clamp(self.x, 0., 1.);
        let g = clamp(self.y, 0., 1.);
        let b = clamp(self.z, 0., 1.);
        RGB{r, g, b}
    }
    pub fn rand_vec(min: f64, max: f64) -> Vec3d {
        let x = rand::thread_rng().gen_range(min, max);
        let y = rand::thread_rng().gen_range(min, max);
        let z = rand::thread_rng().gen_range(min, max);
        Vec3d{x, y, z}
    }
    pub fn rand_in_unit_sphere() -> Vec3d {
        loop {
            let p = Vec3d::rand_vec(-1., 1.);
            if p.len() < 1. {
                return p;
            }
        }
    }
    pub fn rand_in_unit_hemisphere(n: Vec3d) -> Vec3d {
        let v = Vec3d::rand_in_unit_sphere();
        if Vec3d::dot(v, n) > 0. {
            return v;
        }
        else {
            return -v;
        }
    }
}

impl ops::Add for Vec3d
{
    type Output = Vec3d;
    fn add(self, other: Vec3d) -> Vec3d
    {
        Vec3d{ x: self.x+other.x, y: self.y+other.y, z: self.z+other.z }
    }
}

impl ops::Sub for Vec3d
{
    type Output = Vec3d;
    fn sub(self, other: Vec3d) -> Vec3d
    {
        Vec3d{ x: self.x-other.x, y: self.y-other.y, z: self.z-other.z }
    }
}

impl ops::Mul<f64> for Vec3d
{
    type Output = Vec3d;
    fn mul(self, s: f64) -> Vec3d
    {
        Vec3d{ x: self.x*s, y: self.y*s, z: self.z*s } 
    }
}

impl ops::Mul<Vec3d> for f64
{
    type Output = Vec3d;
    fn mul(self, v: Vec3d) -> Vec3d
    {
        Vec3d{ x: v.x*self, y: v.y*self, z: v.z*self } 
    }
}

impl ops::Mul<Vec3d> for Vec3d
{
    type Output = Vec3d;
    fn mul(self, v: Vec3d) -> Vec3d
    {
        Vec3d{ x: v.x*self.x, y: v.y*self.y, z: v.z*self.z } 
    }
}

impl ops::Div<f64> for Vec3d
{
    type Output = Vec3d;
    fn div(self, s: f64) -> Vec3d
    {
        Vec3d{ x: self.x/s, y: self.y/s, z: self.z/s } 
    }
}

impl ops::Neg for Vec3d
{
    type Output = Vec3d;
    fn neg(self) -> Vec3d
    {
        Vec3d{ x: -self.x, y: -self.y, z: -self.z }
    }
}

impl ops::Index<usize> for Vec3d
{
    type Output = f64;
    fn index(&self, i: usize) -> &f64
    {
        match i 
        {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
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