use std::sync::Arc;

use slint::ComponentHandle;
use tokio::sync::RwLock;

use crate::tablet;
use crate::ui;

pub struct AppController {
    pub app_window: ui::AppWindow,
    pub tablet: Arc<RwLock<tablet::Tablet>>,
}

impl AppController {
    pub fn run(self) -> Result <(), slint::PlatformError> {
        self.app_window.run()
    }
}