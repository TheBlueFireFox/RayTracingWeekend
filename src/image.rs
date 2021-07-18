use crate::cvec;

pub type Color = cvec::Color<f64>;

pub struct Image<'a> {
    pixels: &'a [Color],
    height: usize,
    width: usize
}

impl<'a> Image<'a> {
    pub fn new(pixels: &'a [Color], height: usize, width: usize) -> Self {
        debug_assert!(pixels.len() == height * width, "incorrect pixel length");

        Self {
            pixels, height, width
        }
    }
    pub fn get_pixels(&self) -> &'_ [Color] {
        self.pixels
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
}

pub trait Render<'a> {
    fn image(&self) -> &Image<'_>;
}
