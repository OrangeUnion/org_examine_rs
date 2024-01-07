use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use crate::{app, http};
use crate::app::org_examination::UpdateExamination;

pub async fn list_examination() -> impl IntoResponse {
    let data = app::org_examination::select_examinations().await;
    (http::headers(), Json(data))
}

pub async fn update_examination(Json(update): Json<UpdateExamination>) -> impl IntoResponse {
    let data = app::org_examination::update_examination(update).await;
    (http::headers(), Json(data))
}

pub async fn check_examination(Json(answers): Json<Vec<i32>>) -> impl IntoResponse {
    let data = app::org_examination::check_examination(answers, 1).await;
    (http::headers(), Json(data))
}

pub async fn router(app_router: Router) -> Router {
    app_router
        .route("/list_examination", get(list_examination))
        .route("/update_examination", post(update_examination))
        .route("/check_examination", post(check_examination))
}