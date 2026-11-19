use axum::{http::StatusCode, Json};

use crate::dto::todos::{CreateTodoRequest, TodoResponse};

pub(crate) async fn list_todos() -> Json<Vec<TodoResponse>> {
    let todos = vec![
        TodoResponse {
            id: 1,
            title: String::from("Learn Axum"),
            completed: false,
        },
    ];

    Json(todos)
}

pub(crate) async fn create_todo(
    Json(input): Json<CreateTodoRequest>,
) -> Result<&'static str, (StatusCode, &'static str)> {
    if input.title.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Title mustn't be empty",
        ));
    }

    Ok("Todo received")
}
