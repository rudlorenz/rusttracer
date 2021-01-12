use crate::structs::ray::Ray;
use crate::structs::vec3::Point3;

use itertools::*;
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
        let min_c_arr = self.min_c.as_array();
        let max_c_arr = self.max_c.as_array();
        let r_origin_arr = r.origin().as_array();
        let r_dir_arr = r.direction().as_array();
        let mut zipped = izip!(
            min_c_arr.iter(),
            max_c_arr.iter(),
            r_origin_arr.iter(),
            r_dir_arr.iter()
        );

        zipped.all(|(min_c, max_c, origin_c, direction_c)| {
            let inv_d = 1f64 / direction_c;
            let mut t0 = (min_c - origin_c) * inv_d;
            let mut t1 = (max_c - origin_c) * inv_d;

            if inv_d < 0. {
                swap(&mut t0, &mut t1);
            }
            let t_min = t_min.max(t0);
            let t_max = t_max.min(t1);

            t_min < t_max
        })
    }

    pub fn bounding_box(lhs: AABB, rhs: AABB) -> AABB {
        let small = Point3::new(
            lhs.min_c.x_.min(rhs.min_c.x_),
            lhs.min_c.y_.min(rhs.min_c.y_),
            lhs.min_c.z_.min(rhs.min_c.z_),
        );
        let big = Point3::new(
            lhs.max_c.x_.max(rhs.max_c.x_),
            lhs.max_c.y_.max(rhs.max_c.y_),
            lhs.max_c.z_.max(rhs.max_c.z_),
        );

        AABB::new(small, big)
    }

    pub fn expand(&mut self, bbox: AABB) {
        self.min_c = Point3::new(
            self.min_c.x_.min(bbox.min_c.x_),
            self.min_c.y_.min(bbox.min_c.y_),
            self.min_c.z_.min(bbox.min_c.z_),
        );
        self.max_c = Point3::new(
            self.max_c.x_.max(bbox.max_c.x_),
            self.max_c.y_.max(bbox.max_c.y_),
            self.max_c.z_.max(bbox.max_c.z_),
        );
    }
}
