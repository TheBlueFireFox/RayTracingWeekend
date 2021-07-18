#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub const BLACK : Color = Color::new(0,0,0);
pub const WHITE : Color = Color::new(255,255,255);

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn r(&self) -> u8 {
        self.r
    }
    pub const fn g(&self) -> u8 {
        self.g
    }
    pub const fn b(&self) -> u8 {
        self.b
    }
}

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
