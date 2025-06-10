// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    error::Error,
    sync::Arc
};
use tokio::sync::RwLock;

mod app_controller;
mod server;
mod tablet;
mod ui;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tablet = Arc::new(RwLock::new(tablet::Tablet::default()));

    let server = server::Server::new(tablet.clone());

    tokio::spawn(async move {
        if let Err(e) = server.run().await {
            println!("Server failed to run: {}", e)
        };
    });

    let app_window = ui::AppWindow::new()?;

    let app_controller = app_controller::AppController {
        tablet,
        app_window: app_window,
    };

    app_controller.app_window.global::<ui::App>().set_name(slint::SharedString::from(env!("CARGO_PKG_NAME")));
    app_controller.app_window.global::<ui::App>().set_version(slint::SharedString::from(env!("CARGO_PKG_VERSION")));

    app_controller.run()?;

    Ok(())
}
