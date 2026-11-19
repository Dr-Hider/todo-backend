use axum::{routing::get, Router};

use crate::handlers::todos::{create_todo, list_todos};

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/todos", get(list_todos).post(create_todo))
}
