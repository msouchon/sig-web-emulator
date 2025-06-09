use std::{error::Error, sync::Arc};

use axum::{extract::{Path, State}, http::StatusCode, routing::{get, post}, Router};
use tokio::sync::RwLock;
use tower_http::{cors::{Any, CorsLayer}, trace::{self, TraceLayer}};
use tracing::Level;

use crate::tablet::{JustifyMode, Tablet};

pub struct Server {
    pub tablet: Arc<RwLock<Tablet>>
}

impl Server {
    pub fn new(tablet: Arc<RwLock<Tablet>>) -> Self {
        Self {
            tablet
        }
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
            .with_state(self.tablet.clone())
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
            )
            .layer(
                CorsLayer::new().allow_origin(Any)
            )
    }
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