use crate::structs::aabb::AABB;
use crate::structs::hitable::{HitList, HitRecord, Hitable};
use crate::structs::ray::Ray;
use rand::Rng;

pub struct BVHNode {
    left: Option<Box<dyn Hitable + Send + Sync>>,
    right: Option<Box<dyn Hitable + Send + Sync>>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(mut hit_list: HitList) -> BVHNode {
        let comparator = |lhs: Box<dyn Hitable + Send + Sync>, rhs: Box<dyn Hitable + Send + Sync>, axis: usize| {
            let lhs_arr = lhs.bounding_box().unwrap().min_c.as_array();
            let rhs_arr = rhs.bounding_box().unwrap().min_c.as_array();
            if lhs_arr[axis] < rhs_arr[axis] {
                (lhs, rhs)
            } else {
                (rhs, lhs)
            }
        };

        let axis = rand::thread_rng().gen_range(0, 2);
        if hit_list.elements.len() == 1 {
            let hit = hit_list.elements.into_iter().next();
            let bbox = hit.as_ref().unwrap().bounding_box().unwrap();
            BVHNode {
                left: hit,
                right: None,
                bbox,
            }
        } else if hit_list.elements.len() == 2 {
            let mut hit_list = hit_list.elements.into_iter();
            let left = hit_list.next().unwrap();
            let right = hit_list.next().unwrap();
            let (min, max) = comparator(left, right, axis);
            let bbox = AABB::bounding_box(max.bounding_box().unwrap(), min.bounding_box().unwrap());

            BVHNode {
                left: Some(min),
                right: Some(max),
                bbox,
            }
        // else elements.len() > 2
        } else {
            hit_list.elements.sort_by(|lhs, rhs| {
                let lhs = lhs.bounding_box().unwrap().min_c.as_array();
                let rhs = rhs.bounding_box().unwrap().min_c.as_array();
                lhs[axis].partial_cmp(&rhs[axis]).unwrap()
            });
            let bbox = hit_list.bounding_box().unwrap();
            let right = hit_list.split_off(hit_list.elements.len() / 2);
            BVHNode {
                left: Some(Box::new(BVHNode::new(hit_list))),
                right: Some(Box::new(BVHNode::new(right))),
                bbox,
            }
        }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.box_hit(r, t_min, t_max) {
            return None;
        }

        let hit_left;
        let hit_right;
        let tt_max;
        if self.left.is_some() {
            hit_left = self.left.as_ref().unwrap().hit(r, t_min, t_max);
            if hit_left.is_some() {
                tt_max = hit_left.as_ref().unwrap().t;
            } else {
                tt_max = t_max;
            }
        } else {
            hit_left = None;
            tt_max = t_max;
        }
        if self.right.is_some() {
            hit_right = self.right.as_ref().unwrap().hit(r, t_min, tt_max);
        } else {
            hit_right = None;
        }

        let result;
        if hit_right.is_some() && hit_left.is_some() {
            result = if hit_left.as_ref().unwrap().t < hit_right.as_ref().unwrap().t {
                hit_left
            } else {
                hit_right
            };
        } else if hit_left.is_some() {
            result = hit_left;
        } else if hit_right.is_some() {
            result = hit_right;
        } else {
            result = None;
        }

        result
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox)
    }
}
