use crate::structs::ray::Ray;
use crate::structs::vec3::Point3;

use std::mem::swap;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min_c: Point3,
    pub max_c: Point3,
}

impl AABB {
    pub fn zero() -> AABB {
        AABB {
            min_c: Point3::zero(),
            max_c: Point3::zero(),
        }
    }

    pub fn new(a: Point3, b: Point3) -> AABB {
        AABB { min_c: a, max_c: b }
    }

    pub fn box_hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let check_hit = |min_c, max_c, origin_c, direction_c| -> bool {
            let inv_d = 1f64 / direction_c;
            let mut t0 = (min_c - origin_c) * inv_d;
            let mut t1 = (max_c - origin_c) * inv_d;

            if inv_d < 0. {
                swap(&mut t0, &mut t1);
            }
            let t_min = f64::max(t_min, t0);
            let t_max = f64::min(t_max, t1);

            t_min < t_max
        };

        check_hit(self.min_c.x_, self.max_c.x_, r.origin().x_, r.direction().x_)
            && check_hit(self.min_c.y_, self.max_c.y_, r.origin().y_, r.direction().y_)
            && check_hit(self.min_c.z_, self.max_c.z_, r.origin().z_, r.direction().z_)
    }

    pub fn bounding_box(lhs: &AABB, rhs: &AABB) -> AABB {
        let small = Point3::new(
            f64::min(lhs.min_c.x_, rhs.min_c.x_),
            f64::min(lhs.min_c.y_, rhs.min_c.y_),
            f64::min(lhs.min_c.z_, rhs.min_c.z_),
        );
        let big = Point3::new(
            f64::max(lhs.max_c.x_, rhs.max_c.x_),
            f64::max(lhs.max_c.y_, rhs.max_c.y_),
            f64::max(lhs.max_c.z_, rhs.max_c.z_),
        );

        AABB::new(small, big)
    }

    pub fn expand(&mut self, bbox: AABB) {
        self.min_c.x_ = f64::min(self.min_c.x_, bbox.min_c.x_);
        self.min_c.y_ = f64::min(self.min_c.y_, bbox.min_c.y_);
        self.min_c.z_ = f64::min(self.min_c.z_, bbox.min_c.z_);

        self.max_c.x_ = f64::max(self.max_c.x_, bbox.max_c.x_);
        self.max_c.y_ = f64::max(self.max_c.y_, bbox.max_c.y_);
        self.max_c.z_ = f64::max(self.max_c.z_, bbox.max_c.z_);
    }
}
