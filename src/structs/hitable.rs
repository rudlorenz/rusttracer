use crate::structs::material::Material;
use crate::structs::ray::Ray;
use crate::structs::sphere::Sphere;
use crate::structs::vec3::{Point3, Vec3};

use rand::prelude::*;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
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
    pub elements_: Vec<Box<dyn Hitable + Send + Sync>>,
}

impl Hitable for HitList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut last_hit: Option<HitRecord> = None;

        for item in &self.elements_ {
            if let Some(hit) = item.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                last_hit = Some(hit);
            }
        }

        last_hit
    }
}

impl HitList {
    pub fn push(&mut self, item: Box<dyn Hitable + Send + Sync>) {
        self.elements_.push(item);
    }

    pub fn random_scene() -> HitList {
        let horizon = Box::new(Sphere::new(
            10000.,
            Vec3::new(0., -10000., 0.),
            //Material::new_metal(&Vec3::new(0.4, 0.4, 0.4), 0.7),
            Material::new_lambertian(Point3::new(0.6, 0.6, 0.6)),
        ));

        let mut result = HitList {
            elements_: vec![horizon],
        };

        let mut rng = rand::thread_rng();

        let some_point = Point3::new(4., 0.2, 0.);

        for a in -11..11 {
            for b in -11..11 {
                let material_index = rng.gen_range(0, 3);
                let point = Point3::new(
                    a as f64 + 0.9 * rng.gen::<f64>(),
                    0.2,
                    b as f64 + 0.9 * rng.gen::<f64>(),
                );
                if (point - some_point).length() > 0.9 {
                    match material_index {
                        0 => {
                            let rand_color = Point3::new(rng.gen(), rng.gen(), rng.gen());
                            let albedo = rand_color * rand_color;
                            result.push(Box::new(Sphere::new(
                                0.2,
                                point,
                                Material::new_lambertian(albedo),
                            )));
                        }
                        1 => {
                            let albedo = Point3::new(
                                rng.gen_range(0.2, 1.),
                                rng.gen_range(0.2, 1.),
                                rng.gen_range(0.2, 1.),
                            );
                            let fuzz = rng.gen();
                            result.push(Box::new(Sphere::new(
                                0.2,
                                point,
                                Material::new_metal(albedo, fuzz),
                            )));
                        }
                        2 => {
                            let diff_idx = rng.gen_range(0., 2.5);
                            result.push(Box::new(Sphere::new(
                                0.2,
                                point,
                                Material::new_dielectric(diff_idx),
                            )));
                        }
                        _ => {
                            panic!("How could this happen?");
                        }
                    }
                }
            }
        }

        result.push(Box::new(Sphere::new(
            1.,
            Point3::new(0., 1., 0.),
            Material::new_dielectric(1.5),
        )));
        result.push(Box::new(Sphere::new(
            1.,
            Point3::new(-4., 1., 0.),
            Material::new_lambertian(Point3::new(0.4, 0.2, 0.1)),
        )));
        result.push(Box::new(Sphere::new(
            1.,
            Point3::new(4., 1., 0.),
            Material::new_metal(Point3::new(0.7, 0.6, 0.5), 0.),
        )));

        result
    }
}
