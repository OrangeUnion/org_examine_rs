use axum::{Json, Router};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde_json::{json, Value};
use crate::{http, log_info};
use crate::app::{org_examine, org_examine_result, org_paper};
use crate::app::org_examine::{InsertExamine, UpdateExamine};
use crate::app::org_examine_result::{CheckResult, UpdateCheck};
use crate::app::org_paper::UpdatePaper;

pub async fn insert_paper(Json(paper): Json<UpdatePaper>) -> impl IntoResponse {
    log_info!("{:?}", paper);
    if paper.title.is_empty() {
        return (http::headers(), Json(0));
    }
    let union_id = match paper.union_id {
        0 => 1,
        _ => paper.union_id
    };
    let data = org_paper::insert_paper(&paper.title, union_id).await;
    (http::headers(), Json(data))
}

pub async fn update_paper(Json(paper): Json<UpdatePaper>) -> impl IntoResponse {
    log_info!("{:?}", paper);
    let data = org_paper::update_paper(paper.id, &paper.title, paper.union_id).await;
    (http::headers(), Json(data))
}

pub async fn update_paper_status(Json(body): Json<Value>) -> impl IntoResponse {
    log_info!("{:?}", body);
    let id = body["id"].as_i64().unwrap_or(0);
    let status = body["status"].as_i64().unwrap_or(0);
    let data = org_paper::update_paper_status(id, status).await;
    (http::headers(), Json(data))
}

pub async fn delete_paper(Path(id): Path<i64>) -> impl IntoResponse {
    log_info!("删除考卷[{}]", id);
    let data = org_paper::delete_paper(id).await;
    (http::headers(), Json(data))
}

pub async fn list_examine() -> impl IntoResponse {
    let data = org_examine::select_examines().await;
    (http::headers(), Json(data))
}

pub async fn insert_examine(Json(examine): Json<InsertExamine>) -> impl IntoResponse {
    log_info!("{:?}", examine);
    if examine.problem.is_empty() {
        return (http::headers(), Json(0));
    }
    let data = org_examine::insert_examine(examine).await;
    (http::headers(), Json(data))
}

pub async fn update_examine(Json(res): Json<UpdateExamine>) -> impl IntoResponse {
    log_info!("{res:?}");
    let data = org_examine::update_examine(res).await;
    (http::headers(), Json(data))
}

pub async fn delete_examine(Path(id): Path<i64>) -> impl IntoResponse {
    log_info!("删除考题[{}]", id);
    let data = org_examine::delete_examine(id).await;
    (http::headers(), Json(data))
}

pub async fn check_examine(Json(res): Json<UpdateCheck>) -> impl IntoResponse {
    log_info!("{res:?}");
    let data = org_examine::check_examine(res).await;
    let result = match data {
        CheckResult::Pass => "考试合格",
        CheckResult::UnPass => "考试不合格",
        CheckResult::Overrun => "次数用尽",
        CheckResult::TimeOut => "考试超时",
        CheckResult::None => "啥都不是",
    }.to_string();
    let json = json!({
        "result": result
    });
    (http::headers(), Json(json))
}

pub async fn delete_examine_result(Path(id): Path<i64>) -> impl IntoResponse {
    log_info!("删除记录[{}]", id);
    let data = org_examine_result::delete_examine_result(id).await;
}

pub async fn router(app_router: Router) -> Router {
    app_router
        .route("/examine_check", post(check_examine))
}

pub async fn auth_router(app_router: Router) -> Router {
    app_router
        .route("/insert_paper", post(insert_paper))
        .route("/update_paper", post(update_paper))
        .route("/delete_paper/:id", get(delete_paper))
        .route("/insert_examine", post(insert_examine))
        .route("/update_examine", post(update_examine))
        .route("/update_paper_status", post(update_paper_status))
        .route("/delete_examine/:id", get(delete_examine))
        .route("/delete_examine_result/:id", get(delete_examine_result))
        .route("/list_examine", get(list_examine))
}