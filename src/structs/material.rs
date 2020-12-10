use crate::random_unit_in_sphere;
use crate::structs::hitable::HitRecord;
use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

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

    pub fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<Ray> {
        match self {
            Material::Lambertian(lamb) => lamb.scatter(r, hit_record),
            Material::Metal(met) => met.scatter(r, hit_record),
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

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo_: Vec3,
    pub fuzz_: f64,
}

impl Metal {
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
