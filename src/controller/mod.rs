use axum::Router;
use axum_session::{SessionConfig, SessionLayer, SessionNullPool, SessionStore};
use tower_http::cors::CorsLayer;
use crate::*;

mod sys_controller;
mod org_controller;
mod test_controller;
mod html_controller;

pub async fn run() {
    let address = "0.0.0.0:8000";
    log_info!("启动参数: {address}");

    let session_config = SessionConfig::default().with_table_name("session_table");
    let session_store = SessionStore::<SessionNullPool>::new(None, session_config).await.unwrap();

    let mut app = Router::new();
    app = sys_controller::router(app).await;
    app = org_controller::router(app).await;
    app = html_controller::router(app).await;
    app = test_controller::router(app).await;
    app = app.layer(CorsLayer::permissive()).layer(SessionLayer::new(session_store));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}