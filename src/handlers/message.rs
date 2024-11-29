use axum::response::IntoResponse;

pub async fn list_message_handler() -> impl IntoResponse {
    "create message"
}

pub async fn send_message_handler() -> impl IntoResponse {
    "send message"
}
