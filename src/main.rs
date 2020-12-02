use std::fmt::Write;
use std::fs::File;
use std::io::{BufWriter, Write as ioWrite};
use std::path::Path;

fn create_image_bitmap(nx: i32, ny: i32) -> String {
    let mut result = format!("P3\n{} {}\n255\n", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2 as f32;
            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;
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
