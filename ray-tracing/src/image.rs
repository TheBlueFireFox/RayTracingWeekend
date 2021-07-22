use crate::cvec;

pub type Color = cvec::Color<f64>;

pub struct Image<'a> {
    pixels: &'a [Color],
    height: usize,
    width: usize,
    sample: usize,
}

impl<'a> Image<'a> {
    pub fn new(pixels: &'a [Color], height: usize, width: usize, sample: usize) -> Self {
        debug_assert!(pixels.len() == height * width, "incorrect pixel length");

        Self {
            pixels,
            height,
            width,
            sample,
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

    /// Get a reference to the image's sample.
    pub fn sample(&self) -> usize {
        self.sample
    }
}

impl<'a> Render<'a> for Image<'a> {
    fn image(&self) -> &Image<'_> {
        self
    }
}

pub trait Render<'a> {
    fn image(&self) -> &Image<'_>;
}
