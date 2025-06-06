use std::sync::Arc;
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

#[derive(Default)]
struct Tablet {
    state: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let tablet = Arc::new(RwLock::new(Tablet::default()));

    let app = Router::new()
        .route("/", get(|| async {"Hello, world!"}))
        .route("/SigWeb/ClearSignature", get(clear_signature))
        .route("/SigWeb/DaysUntilCertificateExpires", get(get_days_until_certificate_expires))
        .route("/SigWeb/DisplayXSize/{value}", post(set_display_x_size))
        .route("/SigWeb/DisplayYSize/{value}", post(set_display_y_size))
        .route("/SigWeb/JustifyMode/{value}", post(set_justify_mode))
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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
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

async fn set_display_x_size(Path(value): Path<u32>) -> StatusCode {
    StatusCode::OK
}

async fn set_display_y_size(Path(value): Path<u32>) -> StatusCode {
    StatusCode::OK
}

async fn set_justify_mode(Path(value): Path<u32>) -> StatusCode {
    StatusCode::OK
}

async fn clear_signature() -> StatusCode {
    StatusCode::OK
}

async fn get_total_points() -> &'static str {
    "0"
}
