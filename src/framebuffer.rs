use raylib::prelude::*;

pub struct Framebuffer {
    image: Image,
    width: i32,
    height: i32,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let background_color = Color::BLACK;

        let raw_image = unsafe {
            raylib::ffi::GenImageColor(width, height, background_color)
        };

        let image = unsafe {
            Image::from_raw(raw_image)
        };

        Self {
            image,
            width,
            height,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.image.clear_background(self.background_color);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        self.set_pixel_color(x, y, self.current_color);
    }

    pub fn set_pixel_color(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.image.draw_pixel(x, y, color);
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn render_to_file(&self, filename: &str) {
        self.image.export_image(filename);
    }

    pub fn get_image(&self) -> &Image {
        &self.image
    }
}