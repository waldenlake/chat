mod config;
mod handlers;

pub use crate::handlers::*;
use axum::{
    routing::{get, patch, post},
    Router,
};
pub use config::AppConfig;
use std::{ops::Deref, sync::Arc};

#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct AppStateInner {
    config: AppConfig,
}

impl AppState {
    pub fn new(confg: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner { config: confg }),
        }
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);

    let api = Router::new()
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler))
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat/:id",
            patch(update_chat_handler)
                .delete(delete_chat_handler)
                .post(send_message_handler),
        )
        .route("/chat/:id/message", get(list_message_handler));

    Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state)
}

async fn index_handler() {}
