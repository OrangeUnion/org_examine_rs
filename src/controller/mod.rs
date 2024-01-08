use axum::Router;
use tower_http::cors::CorsLayer;
use crate::*;

mod sys_controller;
mod org_controller;
mod test_controller;
mod html_controller;

pub async fn run() {
    let port = app::get_config().await.server_port;
    let address = format!("0.0.0.0:{port}");
    log_info!("启动参数: {address}");

    let mut app = Router::new();
    app = sys_controller::router(app).await;
    app = org_controller::router(app).await;
    app = html_controller::router(app).await;
    app = test_controller::router(app).await;
    app = app.layer(CorsLayer::permissive());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}