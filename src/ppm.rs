use crate::image::Render;
use std::{fmt::Write, fs, io, path::Path};

pub fn save<'a, T: Render<'a>, P: AsRef<Path>>(image: T, path: P) -> Result<(), io::Error> {
    let img = image.image();

    const LINE_LENGTH: usize = 3 * 3 + 3; // => 255 255 255\n

    let text_length = {
        let mut len = 3; // P3\n
        len += ((img.get_width() as f64).log10() + 1.0) as usize;
        len += 1;
        len += ((img.get_height() as f64).log10() + 1.0) as usize;
        len += 5; // => \n255\n
        len += img.get_height() + img.get_height() * img.get_width() * LINE_LENGTH;
        len
    };

    let mut s = String::with_capacity(text_length);

    // no error possible as per docs
    write!(s, "P3\n{} {}\n255\n", img.get_width(), img.get_height()).unwrap();

    for j in 0..img.get_height() {
        for i in 0..img.get_width() {
            let p = &img.get_pixels()[j * img.get_height() + i];
            // no error possible as per docs
            write!(s, "{} {} {}\n", p.r(), p.g(), p.b()).unwrap();
        }
    }

    // write file content
    fs::write(path, &s)
}
