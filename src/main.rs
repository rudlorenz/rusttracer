mod structs;

use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

extern crate image;

use image::Rgb;

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = Vec3::dot(&r.direction(), &r.direction());
    let b = 2.0 * Vec3::dot(&oc, &r.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;

    let discr = b * b - 4. * a * c;

    if discr >= 0. {
        (-b - discr.sqrt()) / (2. * a)
    } else {
        -1.
    }
}

fn ray_col(r: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0., 0., -1.), 0.5, r);
    if t > 0. {
        let normal = Vec3::unit_vector(&(r.point_at(t) - Vec3::new(0., 0., -1.)));
        0.5 * Vec3::new(normal.x_ + 1., normal.y_ + 1., normal.z_ + 1.)
    }
    else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y_ + 1.);

        (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 800;
    let ny = 400;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut img_buf = image::ImageBuffer::new(nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = (ny - j) as f64 / ny as f64;
            let r = Ray::new(
                &origin,
                &(lower_left_corner + u * horizontal + v * vertical),
            );
            let col = ray_col(&r);

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
