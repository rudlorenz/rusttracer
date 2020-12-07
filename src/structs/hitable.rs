use crate::structs::material::Material;
use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub t_: f64,
    pub p_: Vec3,
    pub normal_: Vec3,
    pub material_: Box<dyn Material>,
}

impl HitRecord {
    pub fn new(t: f64, p: &Vec3, normal: &Vec3, material: Box<dyn Material>) -> HitRecord {
        HitRecord {
            t_: t,
            p_: *p,
            normal_: *normal,
            material_: material,
        }
    }
}

pub struct HitList {
    pub elements_: Vec<Box<dyn Hitable + Send + Sync>>,
}

impl Hitable for HitList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut last_hit: Option<HitRecord> = None;

        for item in &self.elements_ {
            if let Some(hit) = item.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t_;
                last_hit = Some(hit);
            }
        }

        last_hit
    }
}
