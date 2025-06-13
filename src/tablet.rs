pub struct Tablet {
    pub state: bool,
    image: tiny_skia::Pixmap,
    last_point_normalized: Option<(f32, f32)>,
}

impl Tablet {
    pub fn default() -> Self {
        let mut image = tiny_skia::Pixmap::new(500, 100).unwrap();
        image.fill(tiny_skia::Color::WHITE);

        Tablet {
            state: false,
            image,
            last_point_normalized: None,
        }
    }

    pub fn draw(
        &mut self,
        normalized_x: f32,
        normalized_y: f32,
        color_rgba: (u8, u8, u8, u8),
        width: f32,
    ) {
        if let Some((last_normalized_x, last_normalized_y)) = self.last_point_normalized {
            use tiny_skia::{LineCap, LineJoin, Paint, PathBuilder, Stroke, Transform};

            let mut paint = Paint::default();
            paint.set_color_rgba8(color_rgba.0, color_rgba.1, color_rgba.2, color_rgba.3);
            paint.anti_alias = true;

            let mut path_builder = PathBuilder::default();
            path_builder.move_to(
                last_normalized_x * self.image.width() as f32,
                last_normalized_y * self.image.height() as f32,
            );
            path_builder.line_to(
                normalized_x * self.image.width() as f32,
                normalized_y * self.image.height() as f32,
            );

            let path = path_builder.finish().unwrap();

            let stroke = Stroke {
                width,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                ..Default::default()
            };

            self.image
                .stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }

        self.last_point_normalized = Some((normalized_x, normalized_y));
    }

    pub fn draw_end(&mut self) {
        self.last_point_normalized = None;
    }

    pub fn draw_start(&mut self, normalized_x: f32, normalized_y: f32) {
        self.last_point_normalized = Some((normalized_x, normalized_y));
    }

    pub fn clear(&mut self) {
        self.image.fill(tiny_skia::Color::WHITE);
        self.last_point_normalized = Some((0.0, 0.0));
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

    pub fn to_png(&self) -> Result<Vec<u8>, png::EncodingError> {
        self.image.encode_png()
    }

    pub fn to_slint_image(&self) -> slint::Image {
        slint::Image::from_rgba8(slint::SharedPixelBuffer::clone_from_slice(
            self.image.data(),
            self.image.width(),
            self.image.height(),
        ))
    }
}
