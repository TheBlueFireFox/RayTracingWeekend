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

    // no error possible as per docs
    for p in img.get_pixels() {
        write!(s, "{} {} {}\n", p.x() as u8, p.y() as u8, p.z() as u8).unwrap();
    }

    // write file content
    fs::write(path, &s)
}

#[cfg(test)]
mod tests {
    use std::{cmp::min, io::{self, Read}};
    use tempfile;

    use crate::{
        image::{Image, Color},
        ppm::save,
    };

    struct Tmp<'a, 'b> {
        img: &'b Image<'a>,
    }

    impl<'a, 'b> Tmp<'a, 'b> {
        fn new(img: &'b Image<'a>) -> Self {
            Self { img }
        }
    }

    impl<'a, 'b> crate::ppm::Render<'b> for Tmp<'a, 'b> {
        fn image(&self) -> &Image<'_> {
            self.img
        }
    }

    #[test]
    fn test_rainbow() -> Result<(), io::Error> {
        const IMAGE_WIDTH: usize = 256;
        const IMAGE_HEIGHT: usize = 256;

        let mut px = vec![Color::new(0.0, 0.0, 0.0); IMAGE_HEIGHT * IMAGE_WIDTH];

        let mut res = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

        let per = |v, c| ((v as f64) / ((c - 1) as f64));

        let rcon = |v| (255.999 * v);
        let con = |v| rcon(v) as u8;

        for j in 0..IMAGE_HEIGHT {
            for i in 0..IMAGE_WIDTH {
                let r = per(i, IMAGE_WIDTH);
                let g = per(255 - j, IMAGE_HEIGHT);
                let b = 0.25;

                px[j * IMAGE_HEIGHT + i] = Color::new(rcon(r), rcon(g), rcon(b));

                res.push_str(&format!("{} {} {}\n", con(r), con(g), con(b)));
            }
        }
        let img = Image::new(&px, IMAGE_HEIGHT, IMAGE_WIDTH);
        let tmp = Tmp::new(&img);

        let tmp_file1 = tempfile::NamedTempFile::new()?;
        let mut tmp_file2 = tmp_file1.reopen()?;

        // write the file back
        save(tmp, tmp_file1)?;

        let mut buf = String::new();
        tmp_file2.read_to_string(&mut buf)?;

        let buf = buf.trim();
        let res = res.trim();

        assert_eq!(buf.len(), res.len());

        const SIZE: usize = 100;

        for i in (0..buf.len()).step_by(SIZE) {
            let b = &buf[i..min(buf.len(), i + SIZE)];
            let r = &res[i..min(res.len(), i + SIZE)];
            assert_eq!(b, r);
        }

        Ok(())
    }
}
