// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    error::Error,
    sync::Arc
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Router,
    routing::{get, post},
};
use tokio::sync::RwLock;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::Level;

slint::include_modules!();

#[derive(Default)]
#[repr(u8)]
enum JustifyMode {
    #[default]
    None = 0,
    TopLeft = 1,
    TopRight = 2,
    BottomLeft = 3,
    BottomRight = 4,
    Center = 5,
}

#[derive(Default)]
struct Tablet {
    display_x_size: u32,
    display_y_size: u32,
    justify_mode: JustifyMode,
    state: bool,
    total_points: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let tablet = Arc::new(RwLock::new(Tablet::default()));

    let app = Router::new()
        .route("/", get(|| async {"Hello, world!"}))
        .route("/SigWeb/ClearSignature", get(clear_signature))
        .route("/SigWeb/DaysUntilCertificateExpires", get(get_days_until_certificate_expires))
        .route("/SigWeb/DisplayXSize/{value}", post(set_display_x_size))
        .route("/SigWeb/DisplayYSize/{value}", post(set_display_y_size))
        .route("/SigWeb/JustifyMode/{value}", post(set_justify_mode))
        .route("/SigWeb/Reset", post(reset))
        .route("/SigWeb/SigWebVersion", get(version))
        .route("/SigWeb/TabletState", get(get_tablet_state))
        .route("/SigWeb/TabletState/{value}", post(set_tablet_state))
        .route("/SigWeb/TotalPoints", get(get_total_points))
        .with_state(tablet)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
        )
        .layer(
            CorsLayer::new().allow_origin(Any)
        );

    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    let ui = AppWindow::new()?;

    ui.global::<App>().set_name(slint::SharedString::from(env!("CARGO_PKG_NAME")));
    ui.global::<App>().set_version(slint::SharedString::from(env!("CARGO_PKG_VERSION")));

    ui.run()?;

    Ok(())
}

async fn get_tablet_state(State(state): State<Arc<RwLock<Tablet>>>) -> &'static str {
    let tablet = state.read().await;
    if tablet.state {"1"} else {"0"}
}

async fn set_tablet_state(State(state): State<Arc<RwLock<Tablet>>>, Path(value): Path<u32>) -> StatusCode {
    let mut tablet = state.write().await;
    tablet.state = value == 1;
    StatusCode::OK
}

async fn version() -> &'static str {
    "\"1.7.2.0\""
}

async fn get_days_until_certificate_expires() -> &'static str {
    "0"
}

async fn set_display_x_size(State(state): State<Arc<RwLock<Tablet>>>, Path(value): Path<u32>) -> StatusCode {
    let mut tablet = state.write().await;
    tablet.display_x_size = value;
    StatusCode::OK
}

async fn set_display_y_size(State(state): State<Arc<RwLock<Tablet>>>, Path(value): Path<u32>) -> StatusCode {
    let mut tablet = state.write().await;
    tablet.display_y_size = value;
    StatusCode::OK
}

async fn set_justify_mode(State(state): State<Arc<RwLock<Tablet>>>, Path(value): Path<u32>) -> StatusCode {
    let mut tablet = state.write().await;
    tablet.justify_mode = match value {
        1 => JustifyMode::TopLeft,
        2 => JustifyMode::TopRight,
        3 => JustifyMode::BottomLeft,
        4 => JustifyMode::BottomRight,
        5 => JustifyMode::Center,
        0 | _ => JustifyMode::None,
    };
    StatusCode::OK
}

async fn clear_signature() -> StatusCode {
    StatusCode::OK
}

async fn get_total_points(State(state): State<Arc<RwLock<Tablet>>>) -> String {
    let tablet = state.read().await;
    tablet.total_points.to_string()
}

async fn reset(State(state): State<Arc<RwLock<Tablet>>>) -> StatusCode {
    let mut tablet = state.write().await;
    *tablet = Tablet::default();
    StatusCode::OK
}
