use crate::vector::Vec3d;

#[derive(Clone, Debug)]
pub struct AABB {
    max: Vec3d,
    min: Vec3d
}

impl AABB {

    pub fn new(p1: Vec3d, p2: Vec3d) -> AABB {
        let min = Vec3d::comp_min(p1, p2);
        let max = Vec3d::comp_max(p1, p2);
        AABB{ min, max }
    }

    pub fn hit(&self, r: &Ray) -> bool {
        let t1 = (self.p_min.x - r.o.x)/r.d.x;
        let t2 = (self.p_max.x - r.o.x)/r.d.x;
        let t3 = (self.p_min.y - r.o.y)/r.d.y;
        let t4 = (self.p_max.y - r.o.y)/r.d.y;
        let t5 = (self.p_min.z - r.o.z)/r.d.z;
        let t6 = (self.p_max.z - r.o.z)/r.d.z;

        let tmin = f64::max(f64::max(f64::min(t1, t2), f64::min(t3, t4)), f64::min(t5, t6));
        let tmax = f64::min(f64::min(f64::max(t1, t2), f64::max(t3, t4)), f64::max(t5, t6));

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
