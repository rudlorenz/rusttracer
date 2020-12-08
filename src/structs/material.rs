use crate::random_unit_in_sphere;
use crate::structs::hitable::HitRecord;
use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

pub trait Material: MaterialClone + Send + Sync {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<Ray>;
    fn attenuation(&self) -> Vec3;
}

// looks hacky as hell
pub trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

// kinda smells too
impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo_: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, hit_record: &HitRecord) -> Option<Ray> {
        let target =
            hit_record.p_ + hit_record.normal_ + random_unit_in_sphere(&mut rand::thread_rng());

        Some(Ray::new(&hit_record.p_, &(target - hit_record.p_)))
    }

    fn attenuation(&self) -> Vec3 {
        // try albedo_ / p_
        self.albedo_
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo_: Vec3,
    pub fuzz_: f64,
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<Ray> {
        let reflect = |v: &Vec3, norm: &Vec3| -> Vec3 { v - 2. * Vec3::dot(v, norm) * norm };

        let reflection = reflect(&Vec3::unit_vector(&r.direction()), &hit_record.normal_);
        let scatter = Ray::new(
            &hit_record.p_,
            &(reflection + self.fuzz_ * random_unit_in_sphere(&mut rand::thread_rng())),
        );
        if Vec3::dot(&scatter.direction(), &hit_record.normal_) > 0. {
            Some(scatter)
        } else {
            None
        }
    }

    fn attenuation(&self) -> Vec3 {
        self.albedo_
    }
}
