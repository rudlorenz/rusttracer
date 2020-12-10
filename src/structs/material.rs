use crate::structs::hitable::HitRecord;
use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

use rand::Rng;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
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

    pub fn new_dielectric(refraction_index: f64) -> Self {
        Material::Dielectric(Dielectric {
            refraction_: refraction_index,
        })
    }

    pub fn scatter<R: Rng>(&self, r: &Ray, hit_record: &HitRecord, rng: &mut R) -> Option<Ray> {
        match self {
            Material::Lambertian(lamb) => lamb.scatter(r, hit_record, rng),
            Material::Metal(met) => met.scatter(r, hit_record, rng),
            Material::Dielectric(diel) => diel.scatter(r, hit_record, rng),
        }
    }

    pub fn attenuation(&self) -> Vec3 {
        match self {
            Material::Lambertian(lamb) => lamb.attenuation(),
            Material::Metal(met) => met.attenuation(),
            Material::Dielectric(_) => Vec3::new(1., 1., 1.),
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

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub refraction_: f64,
}

impl Dielectric {
    fn scatter<R: Rng>(&self, r: &Ray, hit_record: &HitRecord, rng: &mut R) -> Option<Ray> {
        let reflect = |v: &Vec3, norm: &Vec3| -> Vec3 { v - 2. * Vec3::dot(v, norm) * norm };
        let refraction_ratio = if hit_record.front_face_ {
            1. / self.refraction_
        } else {
            self.refraction_
        };

        let unit_dir = Vec3::unit_vector(&r.direction());

        let cos_theta = Vec3::dot(&(-unit_dir), &hit_record.normal_).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen() {
                reflect(&unit_dir, &hit_record.normal_)
            } else {
                self.refract(&unit_dir, &hit_record.normal_, refraction_ratio)
            };

        Some(Ray::new(&hit_record.p_, &direction))
    }

    fn refract(&self, uv: &Vec3, n: &Vec3, eta: f64) -> Vec3 {
        // minimum between dot product and 1.
        let cos_theta = Vec3::dot(&(-uv), &n).min(1.);
        let out_orthogonal = eta * (uv + cos_theta * n);
        let out_parallel = -(1. - Vec3::dot(&out_orthogonal, &out_orthogonal))
            .abs()
            .sqrt();

        out_orthogonal + out_parallel * n
    }

    // Using Schlick's approximation.
    fn reflectance(cosi: f64, refraction_index: f64) -> f64 {
        let r0 = (1. - refraction_index) / (1. + refraction_index).powf(2.);

        r0 + (1. - r0) * (1. - cosi).powf(5.)
    }
}
