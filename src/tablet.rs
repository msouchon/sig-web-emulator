pub struct Tablet {
    pub state: bool,
    image: image::RgbaImage,
    last_pixel: Option<(i32, i32)>,
}

impl Tablet {
    pub fn default() -> Self {
        let mut image = image::RgbaImage::new(500, 100);
        image.fill(255);

        Tablet {
            state: false,
            image,
            last_pixel: None,
        }
    }

    pub fn draw(&mut self, x: i32, y: i32, pixel: image::Rgba<u8>) {
        if let Some((last_x, last_y)) = self.last_pixel {
            imageproc::drawing::draw_antialiased_line_segment_mut(
                &mut self.image,
                (last_x, last_y),
                (x, y),
                pixel,
                imageproc::pixelops::interpolate
            );
        }
        self.last_pixel = Some((x, y));
    }

    pub fn draw_end(&mut self) {
        self.last_pixel = None;
    }

    pub fn draw_start(&mut self, x: i32, y: i32) {
        self.last_pixel = Some((x, y));
    }

    pub fn clear(&mut self) {
        self.image.fill(255);
        self.last_pixel = Some((0, 0));
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