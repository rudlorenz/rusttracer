use crate::structs::aabb::AABB;
use crate::structs::material::Material;
use crate::structs::ray::Ray;
use crate::structs::vec3::{Point3, Vec3};

pub trait Hitable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
}

pub struct HitRecord {
    pub front_face: bool,
    pub t: f64,
    pub hit_point: Point3,
    pub out_normal: Vec3,
    pub material: Material,
}

impl HitRecord {
    pub fn new(
        t: f64,
        hit_point: Point3,
        normal: Vec3,
        ray_direction: Vec3,
        material: Material,
    ) -> HitRecord {
        let front_face = Vec3::dot(&ray_direction, &normal) < 0.;
        let out_normal = if front_face { normal } else { -normal };
        HitRecord {
            front_face,
            t,
            hit_point,
            out_normal,
            material,
        }
    }
}

pub struct HitList {
    pub elements: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut last_hit: Option<HitRecord> = None;

        for item in &self.elements {
            if let Some(hit) = item.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                last_hit = Some(hit);
            }
        }

        last_hit
    }

    fn bounding_box(&self) -> Option<AABB> {
        if self.elements.is_empty() {
            return None;
        }

        self.elements
            .iter()
            .try_fold(AABB::zero(), |mut acc, item| {
                if let Some(bbox) = item.bounding_box() {
                    acc.expand(bbox);
                    Some(acc)
                } else {
                    None
                }
            })
    }
}

impl HitList {
    pub fn with_capacity(capacity: usize) -> HitList {
        HitList {
            elements: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: Box<dyn Hitable>) {
        self.elements.push(item);
    }

    pub fn split_off(&mut self, at: usize) -> Self {
        HitList {
            elements: self.elements.split_off(at),
        }
    }
}
