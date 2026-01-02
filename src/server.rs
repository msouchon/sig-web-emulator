use std::{error::Error, sync::Arc};

use axum::{
    Router,
    body::Body,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use parking_lot::RwLock;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::Level;

use crate::tablet::Tablet;

pub struct Server {
    tablet: Arc<RwLock<Tablet>>,
}

impl Server {
    pub fn new(tablet: Arc<RwLock<Tablet>>) -> Self {
        Self { tablet }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        tracing_subscriber::fmt::init();
        let app = self.setup_router();
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
        axum::serve(listener, app).await?;
        Ok(())
    }

    fn setup_router(&self) -> Router {
        Router::new()
            .route("/", get(|| async { "Hello, world!" }))
            .route("/SigWeb/ClearSignature", get(clear_signature))
            .route(
                "/SigWeb/DaysUntilCertificateExpires",
                get(get_days_until_certificate_expires),
            )
            .route("/SigWeb/Reset", post(reset))
            .route("/SigWeb/SigImage/{value}", get(get_image))
            .route("/SigWeb/SigWebVersion", get(version))
            .route("/SigWeb/TabletState", get(get_tablet_state))
            .route("/SigWeb/TabletState/{value}", post(set_tablet_state))
            .route("/SigWeb/TabletComTest/{value}", post(ok_stub))
            .route("/SigWeb/JustifyMode/{value}", post(ok_stub))
            .route("/SigWeb/DisplayXSize/{value}", post(ok_stub))
            .route("/SigWeb/DisplayYSize/{value}", post(ok_stub))
            .route("/SigWeb/ImageXSize/{value}", post(ok_stub))
            .route("/SigWeb/ImageYSize/{value}", post(ok_stub))
            .route("/SigWeb/TotalPoints", get(get_total_points))
            .with_state(self.tablet.clone())
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            )
            .layer(CorsLayer::new().allow_origin(Any))
    }
}

async fn ok_stub() -> StatusCode {
    StatusCode::OK
}

async fn get_tablet_state(State(state): State<Arc<RwLock<Tablet>>>) -> &'static str {
    let tablet = state.read();
    if tablet.state { "1" } else { "0" }
}

async fn set_tablet_state(
    State(state): State<Arc<RwLock<Tablet>>>,
    Path(value): Path<u32>,
) -> StatusCode {
    let mut tablet = state.write();
    tablet.state = value == 1;
    StatusCode::OK
}

async fn version() -> &'static str {
    "\"1.7.2.0\""
}

async fn get_days_until_certificate_expires() -> &'static str {
    "0"
}

async fn clear_signature(State(state): State<Arc<RwLock<Tablet>>>) -> StatusCode {
    let mut tablet = state.write();
    tablet.clear();
    StatusCode::OK
}

async fn get_total_points(State(state): State<Arc<RwLock<Tablet>>>) -> String {
    let tablet = state.read();
    tablet.total_points().to_string()
}

async fn reset(State(state): State<Arc<RwLock<Tablet>>>) -> StatusCode {
    let mut tablet = state.write();
    tablet.state = false;
    tablet.clear();
    StatusCode::OK
}

async fn get_image(State(state): State<Arc<RwLock<Tablet>>>) -> impl IntoResponse {
    let tablet = state.read();

    let bytes = match tablet.to_png() {
        Ok(bytes) => bytes,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error generating image: {}", e),
            ));
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/png")
        .body(Body::from(bytes))
        .unwrap())
}
