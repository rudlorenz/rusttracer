mod structs;

use crate::structs::ray::Ray;
use crate::structs::vec3::Vec3;

use std::fmt::Write;
use std::fs::File;
use std::io::{BufWriter, Write as ioWrite};
use std::path::Path;

fn create_image_bitmap(nx: i32, ny: i32) -> String {
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut result = format!("P3\n{} {}\n255\n", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::new(
                &origin,
                &(lower_left_corner + u * horizontal + v * vertical),
            );
            let col = Ray::color(&r);

            let ir = (255.99 * col.x_) as i32;
            let ig = (255.99 * col.y_) as i32;
            let ib = (255.99 * col.z_) as i32;
            writeln!(result, "{} {} {}", ir, ig, ib);
        }
    }

    result
}

fn main() {
    let path = Path::new("image.ppm");

    let file_handle = match File::create(&path) {
        Err(why) => panic!("Couldn't create {} : {}", path.display(), why),
        Ok(file) => file,
    };
    let mut file_writer = BufWriter::new(file_handle);

    match file_writer.write_all(create_image_bitmap(400, 400).as_bytes()) {
        Err(why) => panic!("Couldn't write to {} : {}", path.display(), why),
        Ok(_) => println!("Done!\n{}\n", path.canonicalize().unwrap().display()),
    }
}
