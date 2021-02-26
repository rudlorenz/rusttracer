use crate::structs::hitable::HitRecord;
use crate::structs::material::Material;
use crate::structs::ray::Ray;
use crate::structs::vec3::{Point3, Vec3};

pub struct Sphere {
    radius: f64,
    center: Point3,
    material: Material,
}

impl Sphere {
    pub fn new(radius: f64, center: Point3, material: Material) -> Sphere {
        Sphere {
            radius,
            center,
            material,
        }
    }

    pub(crate) fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discr = b.powf(2f64) - a * c;
        if discr > 0. {
            let root = (-b - discr.sqrt()) / a;
            if root > t_min && root < t_max {
                return Some(HitRecord::new(
                    root,
                    r.point_at(root),
                    (r.point_at(root) - self.center) / self.radius,
                    r.direction(),
                    self.material,
                ));
            }
            let root = (-b + discr.sqrt()) / a;
            if root > t_min && root < t_max {
                return Some(HitRecord::new(
                    root,
                    r.point_at(root),
                    (r.point_at(root) - self.center) / self.radius,
                    r.direction(),
                    self.material,
                ));
            }
        }
        None
    }
}
