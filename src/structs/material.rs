use crate::structs::hitable::HitRecord;
use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

use rand::Rng;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn new_lambertian(albedo: &Vec3) -> Self {
        Material::Lambertian(Lambertian { albedo_: *albedo })
    }

    pub fn new_metal(albedo: &Vec3, fuzz: f64) -> Self {
        Material::Metal(Metal {
            albedo_: *albedo,
            fuzz_: fuzz,
        })
    }

    pub fn scatter<R: Rng>(&self, r: &Ray, hit_record: &HitRecord, rng: &mut R) -> Option<Ray> {
        match self {
            Material::Lambertian(lamb) => lamb.scatter(r, hit_record, rng),
            Material::Metal(met) => met.scatter(r, hit_record, rng),
        }
    }

    pub fn attenuation(&self) -> Vec3 {
        match self {
            Material::Lambertian(lamb) => lamb.attenuation(),
            Material::Metal(met) => met.attenuation(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo_: Vec3,
}

impl Lambertian {
    fn scatter<R: Rng>(&self, _r: &Ray, hit_record: &HitRecord, rng: &mut R) -> Option<Ray> {
        let scatter_dir = hit_record.normal_ + Vec3::random_in_hemisphere(&hit_record.normal_, rng);

        Some(Ray::new(&hit_record.p_, &scatter_dir))
    }

    fn attenuation(&self) -> Vec3 {
        // try albedo_ / p_
        self.albedo_
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo_: Vec3,
    pub fuzz_: f64,
}

impl Metal {
    fn scatter<R: Rng>(&self, r: &Ray, hit_record: &HitRecord, rng: &mut R) -> Option<Ray> {
        let reflect = |v: &Vec3, norm: &Vec3| -> Vec3 { v - 2. * Vec3::dot(v, norm) * norm };

        let reflection = reflect(&Vec3::unit_vector(&r.direction()), &hit_record.normal_);
        let scatter = Ray::new(
            &hit_record.p_,
            &(reflection + self.fuzz_ * Vec3::random_in_unit_sphere(rng)),
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
