use axum::Router;

use crate::routes::health;

pub fn build_router() -> Router {
    Router::new()
        .merge(health::routes())
}
