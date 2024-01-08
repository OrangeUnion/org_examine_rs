use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use crate::{app, http, log_info};
use crate::app::org_examine::UpdateExamine;
use crate::app::org_examine_paper::{UpdatePaper};

pub async fn list_examine() -> impl IntoResponse {
    let data = app::org_examine::select_examines().await;
    (http::headers(), Json(data))
}

pub async fn update_examine(Json(update): Json<UpdateExamine>) -> impl IntoResponse {
    let data = app::org_examine::update_examine(update).await;
    (http::headers(), Json(data))
}

pub async fn check_examine(Json(answers): Json<Vec<i32>>) -> impl IntoResponse {
    let data = app::org_examine::check_examine(answers, 1).await;
    (http::headers(), Json(data))
}

pub async fn update_paper(Json(paper): Json<UpdatePaper>) -> impl IntoResponse {
    log_info!("{:?}", paper);
    let data = app::org_examine_paper::update_examine_paper(paper.id, &paper.title, paper.union_id).await;
    (http::headers(), Json(data))
}

pub async fn router(app_router: Router) -> Router {
    app_router
        .route("/list_examine", get(list_examine))
        .route("/update_examine", post(update_examine))
        .route("/check_examine", post(check_examine))
        .route("/update_paper", post(update_paper))
}