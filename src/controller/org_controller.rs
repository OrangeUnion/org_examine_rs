use axum::{Json, Router};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde_json::Value;
use crate::{http, log_info};
use crate::app::{org_examine, org_paper};
use crate::app::org_examine::{InsertExamine, UpdateExamine};
use crate::app::org_paper::UpdatePaper;

pub async fn list_examine() -> impl IntoResponse {
    let data = org_examine::select_examines().await;
    (http::headers(), Json(data))
}

pub async fn insert_examine(Json(examine): Json<InsertExamine>) -> impl IntoResponse {
    log_info!("{:?}", examine);
    if examine.problem.is_empty() {
        return (http::headers(), Json(0));
    }
    let data = org_examine::insert_examine(examine.problem, examine.paper_id).await;
    (http::headers(), Json(data))
}

pub async fn update_examine(Json(res): Json<Value>) -> impl IntoResponse {
    log_info!("{res}");
    let mut vec_answers = vec![];
    for answer in res["answers"].as_array().unwrap() {
        vec_answers.push(answer.as_str().unwrap_or("").to_string())
    };
    let update = UpdateExamine {
        id: res["id"].as_str().unwrap_or("0").parse::<i64>().unwrap_or(0),
        problem: res["problem"].as_str().unwrap_or("").to_string(),
        answers: vec_answers,
        correct_answer: res["correct_answer"].as_str().unwrap_or("0").parse::<i64>().unwrap_or(0),
        problem_type: 1,
        paper_id: res["paper_id"].as_str().unwrap_or("0").parse::<i64>().unwrap_or(0),
    };
    let data = org_examine::update_examine(update).await;
    (http::headers(), Json(data))
}

pub async fn delete_examine(Path(id): Path<i64>) -> impl IntoResponse {
    log_info!("删除考题[{}]", id);
    let data = org_examine::delete_examine(id).await;
    (http::headers(), Json(data))
}

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

pub async fn delete_paper(Path(id): Path<i64>) -> impl IntoResponse {
    log_info!("删除考卷[{}]", id);
    let data = org_paper::delete_paper(id).await;
    (http::headers(), Json(data))
}

pub async fn router(app_router: Router) -> Router {
    app_router
        .route("/list_examine", get(list_examine))
        .route("/insert_examine", post(insert_examine))
        .route("/update_examine", post(update_examine))
        .route("/delete_examine/:id", get(delete_examine))
        .route("/insert_paper", post(insert_paper))
        .route("/update_paper", post(update_paper))
        .route("/delete_paper/:id", get(delete_paper))
}