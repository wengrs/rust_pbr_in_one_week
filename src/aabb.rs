use crate::vector::Vec3d;
use crate::ray::Ray;

#[derive(Clone, Debug)]
pub struct AABB {
    max: Vec3d,
    min: Vec3d
}

impl AABB {

    pub fn zero() -> AABB {
        let min = Vec3d::zero();
        let max = Vec3d::zero();
        AABB{ min, max }
    }

    pub fn new(p1: Vec3d, p2: Vec3d) -> AABB {
        let min = Vec3d::comp_min(p1, p2);
        let max = Vec3d::comp_max(p1, p2);
        AABB{ min, max }
    }

    pub fn union_box(b1: &AABB, b2: &AABB) -> AABB
    {
        let p_min = Vec3d::comp_min(b1.min, b2.min);
        let p_max = Vec3d::comp_max(b1.max, b2.max);
        AABB::new(p_min, p_max)
    }
    
    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> bool {
        let t1 = (self.min.x - r.ori.x)/r.dir.x;
        let t2 = (self.max.x - r.ori.x)/r.dir.x;
        let t3 = (self.min.y - r.ori.y)/r.dir.y;
        let t4 = (self.max.y - r.ori.y)/r.dir.y;
        let t5 = (self.min.z - r.ori.z)/r.dir.z;
        let t6 = (self.max.z - r.ori.z)/r.dir.z;

        let tmin = f64::max(f64::max(f64::max(f64::min(t1, t2), f64::min(t3, t4)), f64::min(t5, t6)), tmin);
        let tmax = f64::min(f64::min(f64::min(f64::max(t1, t2), f64::max(t3, t4)), f64::max(t5, t6)), tmax);

        if tmax < 0.
        {
            return false;
        }
        else if tmin > tmax
        {
            return false;
        }
        else
        {
            return true;
        }
    }
}
