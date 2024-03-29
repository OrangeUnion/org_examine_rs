use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use tower_http::cors::CorsLayer;
use crate::*;
use crate::controller::auths::TokenAuth;

mod sys_controller;
mod org_controller;
mod test_controller;
mod html_controller;
mod auths;

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404")
}

pub async fn run() {
    let port = app::get_config().await.server_port;
    let address = format!("0.0.0.0:{port}");
    log_info!("启动参数: {address}");

    let mut app = Router::new()
        .fallback(handler_404);

    app = html_controller::auth_router(app).await;
    app = sys_controller::auth_router(app).await;
    app = org_controller::auth_router(app).await;
    app = app.layer(AsyncRequireAuthorizationLayer::new(TokenAuth));

    app = html_controller::router(app).await;
    app = sys_controller::router(app).await;
    app = org_controller::router(app).await;
    app = test_controller::router(app).await;
    app = app.layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}