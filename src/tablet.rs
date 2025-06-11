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

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: image::Rgba<u8>) {
        if x < self.image.width() && y < self.image.height() {
            self.image.put_pixel(x, y, pixel);
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