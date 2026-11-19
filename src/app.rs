use axum::Router;

use crate::routes::health;
use crate::routes::todos;

pub fn build_router() -> Router {
    Router::new()
        .merge(health::routes())
        .merge(todos::routes())
}
