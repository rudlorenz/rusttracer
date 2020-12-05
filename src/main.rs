mod structs;

use crate::structs::hitable::*;
use crate::structs::ray::Ray;
use crate::structs::sphere::Sphere;
use crate::structs::vec3::Vec3;

extern crate image;

use image::Rgb;

fn ray_col(r: &Ray, world: &HitList) -> Vec3 {
    let tryhit = world.hit(r, 0., f64::MAX);

    match tryhit {
        Some(hit_rec) => 0.5 * (hit_rec.normal_ + 1.),
        None => {
            let t = 0.5 * (Vec3::unit_vector(&r.direction()).y_ + 1.);
            (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let nx = 800;
    let ny = 400;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let hit_listy = HitList {
        elements_: vec![
            Box::new(Sphere::new(0.5, &Vec3::new(0., 0., -1.))),
            Box::new(Sphere::new(100., &Vec3::new(0., -100.5, -1.))),
        ],
    };

    let mut img_buf = image::ImageBuffer::new(nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = (ny - j) as f64 / ny as f64;
            let r = Ray::new(
                &origin,
                &(lower_left_corner + u * horizontal + v * vertical),
            );
            let col = ray_col(&r, &hit_listy);

            let ir = (255.99 * col.x_) as u8;
            let ig = (255.99 * col.y_) as u8;
            let ib = (255.99 * col.z_) as u8;

            img_buf.put_pixel(i, j, Rgb([ir, ig, ib]));
        }
    }

    match img_buf.save("result.png") {
        Err(why) => println!("Unable to save result.png : {}", why),
        Ok(_) => println!("Done!"),
    };
}
