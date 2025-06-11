pub struct Tablet {
    pub state: bool,
    image: tiny_skia::Pixmap,
    last_pixel: Option<(i32, i32)>,
}

impl Tablet {
    pub fn default() -> Self {
        let mut image = tiny_skia::Pixmap::new(500, 100).unwrap();
        image.fill(tiny_skia::Color::WHITE);

        Tablet {
            state: false,
            image,
            last_pixel: None,
        }
    }

    pub fn draw(&mut self, x: i32, y: i32, color_rgba: (u8, u8, u8, u8)) {
        if let Some((last_x, last_y)) = self.last_pixel {
            use tiny_skia::{
                LineCap,
                LineJoin,
                Paint,
                PathBuilder,
                Stroke,
                Transform,
            };

            let mut paint = Paint::default();
            paint.set_color_rgba8(
                color_rgba.0,
                color_rgba.1,
                color_rgba.2,
                color_rgba.3
            );
            paint.anti_alias = true;

            let mut path_builder = PathBuilder::default();
            path_builder.move_to(last_x as f32, last_y as f32);
            path_builder.line_to(x as f32, y as f32);

            let path = path_builder.finish().unwrap();

            let stroke = Stroke {
                width: 5.0,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                ..Default::default()
            };

            self.image.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
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
        self.image.fill(tiny_skia::Color::WHITE);
        self.last_pixel = Some((0, 0));
    }

    pub fn total_points(&self) -> u32 {
        let mut total_points = 0;

        for pixel in self.image.pixels() {
            if pixel.demultiply() != tiny_skia::Color::WHITE.to_color_u8() {
                total_points += 1;
            }
        }

        total_points
    }

    pub fn to_png(&self) -> Result<Vec<u8>, png::EncodingError>  {
        self.image.encode_png()
    }

    pub fn to_slint_image(&self) -> slint::Image {
        slint::Image::from_rgba8(slint::SharedPixelBuffer::clone_from_slice(self.image.data(), self.image.width(), self.image.height()))
    }
}