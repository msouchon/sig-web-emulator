use std::sync::{Arc, Mutex};

use parking_lot::RwLock;
use slint::ComponentHandle;

use crate::{tablet, ui};

pub struct AppController {
    pub app_window: ui::AppWindow,
    pub tablet: Arc<RwLock<tablet::Tablet>>,
    last_pixel: Arc<Mutex<Option<(i32, i32)>>>,
}

impl AppController {
    pub fn new(app_window: ui::AppWindow, tablet: Arc<RwLock<tablet::Tablet>>) -> Self {
        Self {
            app_window,
            tablet,
            last_pixel: Arc::new(Mutex::new(None)),
        }
    }

    pub fn run(self) -> Result <(), slint::PlatformError> {
        self.app_window.set_image(self.tablet.read().to_slint_image());

        {
            let app_window_weak = self.app_window.as_weak();
            let tablet = self.tablet.clone();
            let last_pixel = self.last_pixel.clone();
            self.app_window.on_clear(move || {
                let tablet = tablet.clone();
                let mut tablet = tablet.write();

                tablet.clear();

                let image = tablet.to_slint_image();

                let app_window_weak = app_window_weak.clone();
                if let Some(app_window) = app_window_weak.upgrade() {
                    app_window.set_image(image);
                }

                let last_pixel = last_pixel.clone();
                let mut last_pixel = last_pixel.lock().unwrap();
                *last_pixel = None;
            });
        }

        {
            let last_pixel = self.last_pixel.clone();
            self.app_window.on_draw_end(move |_, _| {
                let last_pixel = last_pixel.clone();
                let mut last_pixel = last_pixel.lock().unwrap();
                *last_pixel = None;
            });
        }

        {
            let last_pixel = self.last_pixel.clone();
            self.app_window.on_draw_start(move |x, y| {
                let last_pixel = last_pixel.clone();
                let mut last_pixel = last_pixel.lock().unwrap();
                *last_pixel = Some((x, y));
            });
        }

        {
            let app_window_weak = self.app_window.as_weak();
            let tablet = self.tablet.clone();
            let last_pixel = self.last_pixel.clone();
            self.app_window.on_draw(move |x, y, color| {
                let tablet = tablet.clone();
                let mut tablet = tablet.write();
                let last_pixel = last_pixel.clone();
                let mut last_pixel = last_pixel.lock().unwrap();

                if let Some((last_x, last_y)) = *last_pixel {
                    let pixel = image::Rgba([color.red(), color.green(), color.blue(), color.alpha()]);
                    tablet.put_line(last_x, last_y, x, y, pixel)
                };

                let image = tablet.to_slint_image();

                let app_window_weak = app_window_weak.clone();
                if let Some(app_window) = app_window_weak.upgrade() {
                    app_window.set_image(image);
                }

                *last_pixel = Some((x, y));
            });
        }

        self.app_window.run()
    }
}