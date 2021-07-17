#[derive(Clone, Copy)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn r(&self) -> u8 {
        self.r
    }
    pub fn g(&self) -> u8 {
        self.g
    }
    pub fn b(&self) -> u8 {
        self.b
    }
}

pub struct Image<'a> {
    pixels: &'a [Pixel],
    height: usize,
    width: usize
}

impl<'a> Image<'a> {
    pub fn new(pixels: &'a [Pixel], height: usize, width: usize) -> Self {
        debug_assert!(pixels.len() == height * width, "incorrect pixel length");

        Self {
            pixels, height, width
        }
    }
    pub fn get_pixels(&self) -> &'_ [Pixel] {
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
