use crate::structs::aabb::AABB;
use crate::structs::hitable::{HitList, HitRecord, Hitable};
use crate::structs::ray::Ray;
use rand::Rng;

enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hitable>),
}

pub struct BVH {
    root: BVHNode,
    bbox: AABB,
}

impl BVH {
    pub fn new(mut hit_list: HitList) -> BVH {
        let axis = rand::thread_rng().gen_range(0, 2);
        if hit_list.elements.len() == 1 {
            let hit = hit_list.elements.pop().unwrap();
            let bbox = hit.bounding_box().unwrap();
            BVH {
                root: BVHNode::Leaf(hit),
                bbox,
            }
        } else {
            hit_list.elements.sort_unstable_by(|lhs, rhs| {
                lhs.as_ref().bounding_box().unwrap().min_c[axis]
                    .partial_cmp(&rhs.as_ref().bounding_box().unwrap().min_c[axis])
                    .unwrap()
            });

            let bbox = hit_list.bounding_box().unwrap();
            let right = BVH::new(hit_list.split_off(hit_list.elements.len() / 2));
            let left = BVH::new(hit_list);
            BVH {
                root: BVHNode::Branch {
                    left: Box::new(left),
                    right: Box::new(right),
                },
                bbox,
            }
        }
    }
}

impl Hitable for BVH {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bbox.box_hit(r, t_min, t_max) {
            match &self.root {
                BVHNode::Leaf(leaf) => leaf.hit(r, t_min, t_max),
                BVHNode::Branch { left, right } => {
                    let ttmax;
                    let left_hit = left.hit(r, t_min, t_max);
                    if left_hit.is_some() {
                        ttmax = left_hit.as_ref().unwrap().t;
                    } else {
                        ttmax = t_max
                    };
                    let right_hit = right.hit(r, t_min, ttmax);
                    if right_hit.is_some() {
                        right_hit
                    } else {
                        left_hit
                    }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox)
    }
}
