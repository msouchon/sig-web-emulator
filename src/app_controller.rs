use std::sync::Arc;

use parking_lot::RwLock;
use slint::ComponentHandle;

use crate::{tablet, ui};

pub struct AppController {
    pub app_window: ui::AppWindow,
    pub tablet: Arc<RwLock<tablet::Tablet>>,
}

impl AppController {
    pub fn new(app_window: ui::AppWindow, tablet: Arc<RwLock<tablet::Tablet>>) -> Self {
        Self {
            app_window,
            tablet,
        }
    }

    pub fn run(self) -> Result <(), slint::PlatformError> {
        let window = self.app_window.as_weak();
        let tablet = self.tablet.clone();

        let window_clear = window.clone();
        let tablet_clear = tablet.clone();
        self.app_window.on_clear(move || {
            let window = window_clear.clone();
            let tablet = tablet_clear.clone();

            let mut tablet = tablet.write();
            tablet.clear();
            
            let image = tablet.to_slint_image();
            if let Some(window) = window.upgrade() {
                window.set_image(image);
            }
        });

        let window_draw = window.clone();
        let tablet_draw = tablet.clone();
        self.app_window.on_draw(move |x, y| {
            let window = window_draw.clone();
            let tablet = tablet_draw.clone();

            let mut tablet = tablet.write();
            tablet.set_pixel(x as u32, y as u32, image::Rgba([255, 0, 0, 255]));
            
            let image = tablet.to_slint_image();
            if let Some(window) = window.upgrade() {
                window.set_image(image);
            }
        });

        self.app_window.run()
    }
}