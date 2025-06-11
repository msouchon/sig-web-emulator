pub struct Tablet {
    pub state: bool,
    image: image::RgbaImage,
}

impl Tablet {
    pub fn default() -> Self {
        let mut image = image::RgbaImage::new(500, 100);
        image.fill(255);

        Tablet {
            state: false,
            image,
        }
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, pixel: image::Rgba<u8>) {
        if x >= 0 && x < self.image.width() as i32 && y >= 0 && y < self.image.height() as i32 {
            self.image.put_pixel(x as u32, y as u32, pixel);
        }
    }

    pub fn put_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, pixel: image::Rgba<u8>) {
        for (x, y) in line_drawing::Bresenham::new((x1, y1), (x2, y2)) {
            self.put_pixel(x, y, pixel);
        }
    }

    pub fn clear(&mut self) {
        self.image.fill(255);
    }

    pub fn total_points(&self) -> u32 {
        let mut total_points = 0;

        for pixel in self.image.pixels() {
            if *pixel != image::Rgba([255, 255, 255, 255]) {
                total_points += 1;
            }
        }

        total_points
    }

    pub fn to_png(&self) -> Result<Vec<u8>, image::ImageError>  {
        let mut bytes: Vec<u8> = Vec::new();
        self.image.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png)?;

        Ok(bytes)
    }

    pub fn to_slint_image(&self) -> slint::Image {
        slint::Image::from_rgba8(slint::SharedPixelBuffer::clone_from_slice(self.image.as_raw(), self.image.width(), self.image.height()))
    }
}