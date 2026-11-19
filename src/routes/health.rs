use axum::{routing::get, Router};

use crate::handlers::health::health;

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/health", get(health))
}
