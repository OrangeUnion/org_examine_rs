use axum::extract::Path;
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde_json::Value;
use crate::{app, http, util};

pub async fn select_users() -> impl IntoResponse {
    let data = app::sys_user::select_all_user().await;
    (http::headers(), Json(data))
}

pub async fn select_user(Path(username): Path<String>) -> impl IntoResponse {
    let data = app::sys_user::select_one_user(&username).await;
    (http::headers(), Json(data))
}

pub async fn select_user_groups(Path(username): Path<String>) -> impl IntoResponse {
    let data = app::sys_group::select_user_groups(&username).await;
    (http::headers(), Json(data))
}

pub async fn list_user(Path(username): Path<String>) -> impl IntoResponse {
    let data = util::permission::list_user(&username).await;
    (http::headers(), Json(data))
}

pub async fn show_user(Path(username): Path<String>) -> impl IntoResponse {
    let data = util::permission::show_user(&username).await;
    (http::headers(), Json(data))
}

pub async fn login_check(Json(body): Json<Value>) -> impl IntoResponse {
    let username = body["username"].as_str().unwrap_or("");
    let password = body["password"].as_str().unwrap_or("");
    let data = app::sys_user::check_login(username, password).await;
    (http::headers(), Json(data))
}

pub async fn logout() -> impl IntoResponse {
    axum::http::Response::builder()
        .status(StatusCode::FOUND) // 设置状态码为 302 Found
        .header("Location", "/login") // 设置重定向的 URL
        .body("Redirecting Login".to_string()) // 可选的响应体内容
        .unwrap()
}

pub async fn router(app_router: Router) -> Router {
    app_router
        .route("/users", get(select_users))
        .route("/login_check", post(login_check))
        .route("/show_user/:username", get(show_user))
        .route("/logout", get(logout))
}

pub async fn auth_router(app_router: Router) -> Router {
    app_router
        .route("/user/:username", get(select_user))
        .route("/group/:username", get(select_user_groups))
        .route("/list_user/:username", get(list_user))
}