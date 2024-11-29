use axum::response::IntoResponse;

pub async fn list_chat_handler() -> impl IntoResponse {
    "chat"
}

pub async fn create_chat_handler() -> impl IntoResponse {
    "create chat"
}

pub async fn update_chat_handler() -> impl IntoResponse {
    "update chat"
}

pub async fn delete_chat_handler() -> impl IntoResponse {
    "delete chat"
}
