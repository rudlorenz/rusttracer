use crate::structs::aabb::AABB;
use crate::structs::hitable::{HitRecord, Hitable};
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
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discr = b * b - a * c;
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
            } else {
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
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        let bbox = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(bbox)
    }
}
