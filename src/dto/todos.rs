use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct TodoResponse {
    pub(crate) id: i64,
    pub(crate) title: String,
    pub(crate) completed: bool,
}

#[derive(Deserialize)]
pub(crate) struct CreateTodoRequest {
    pub(crate) title: String,
}
