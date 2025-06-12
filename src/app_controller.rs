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
        self.app_window.set_image(self.tablet.read().to_slint_image());

        {
            let app_window_weak = self.app_window.as_weak();
            let tablet = self.tablet.clone();
            self.app_window.on_clear(move || {
                let tablet = tablet.clone();
                let mut tablet = tablet.write();

                tablet.clear();

                let image = tablet.to_slint_image();

                let app_window_weak = app_window_weak.clone();
                if let Some(app_window) = app_window_weak.upgrade() {
                    app_window.set_image(image);
                }
            });
        }

        {
            let app_window_weak = self.app_window.as_weak();
            let tablet = self.tablet.clone();
            self.app_window.on_draw(move |normalized_x, normalized_y, color, width| {
                let tablet = tablet.clone();
                let mut tablet = tablet.write();

                tablet.draw(normalized_x, normalized_y, (color.red(), color.green(), color.blue(), color.alpha()), width);

                let image = tablet.to_slint_image();

                let app_window_weak = app_window_weak.clone();
                if let Some(app_window) = app_window_weak.upgrade() {
                    app_window.set_image(image);
                }
            });

        {
            let tablet = self.tablet.clone();
            self.app_window.on_draw_end(move |_, _| {
                let tablet = tablet.clone();
                let mut tablet = tablet.write();
                tablet.draw_end();
            });
        }

        {
            let tablet = self.tablet.clone();
            self.app_window.on_draw_start(move |normalized_x, normalized_y| {
                let tablet = tablet.clone();
                let mut tablet = tablet.write();
                tablet.draw_start(normalized_x, normalized_y);
            });
        }
        }

        self.app_window.run()
    }
}