use std::f32::consts::E;
use axum::handler::HandlerWithoutStateExt;
use axum::http::{HeaderMap, Request, Response, StatusCode, Uri};
use axum::response::{IntoResponse, Redirect};
use axum::{Extension, Router};
use axum::body::Body;
use futures_util::future::BoxFuture;
use tower::Service;
use tower_http::auth::AsyncAuthorizeRequest;
use tower_http::cors::CorsLayer;
use crate::*;
use crate::app::redis_util;

mod sys_controller;
mod org_controller;
mod test_controller;
mod html_controller;
mod auths;

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404")
}

async fn handler_to_login(headers: Extension<HeaderMap>) -> impl IntoResponse {
    let header_token = headers.0.get("token").unwrap();
    let header_user = headers.0.get("user").unwrap();
    let token = header_token.to_str().unwrap_or("");
    let user = header_user.to_str().unwrap_or("");
    if redis_util::RedisUserInfo::redis_get_session(user).await.token_eq(token) {
        return Redirect::to("/");
    }
    Redirect::to("/login")
}

pub async fn run() {
    let port = app::get_config().await.server_port;
    let address = format!("0.0.0.0:{port}");
    log_info!("启动参数: {address}");

    let mut app = Router::new()
        .fallback(handler_404);

    app = html_controller::router(app).await;
    app = sys_controller::router(app).await;
    app = org_controller::router(app).await;
    app = test_controller::router(app).await;
    app = app.layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}