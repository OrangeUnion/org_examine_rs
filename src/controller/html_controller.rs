use askama_axum::IntoResponse;
use axum::response::Html;
use axum::{Form, Router};
use axum::extract::Path;
use axum::routing::{get, post};
use serde_json::Value;
use crate::{app, http, log_info, log_link};
use crate::app::org_examine::check_examine;

pub async fn index() -> impl IntoResponse {
    let template = http::IndexTemplate {
        title: "首页".to_string(),
    }.to_string();
    Html(template)
}

pub async fn login() -> impl IntoResponse {
    let template = http::LoginTemplate {
        title: "登录".to_string()
    }.to_string();
    Html(template)
}

pub async fn examine_client() -> impl IntoResponse {
    let template = http::ExamineClientTemplate {
        title: "考试题目".to_string(),
        examines: app::org_examine::select_examines_by_paper().await,
    }.to_string();
    Html(template)
}

pub async fn examine_update() -> impl IntoResponse {
    let template = http::ExamineUpdateTemplate {
        title: "考题配置".to_string()
    }.to_string();
    Html(template)
}

pub async fn examine_check(Form(res): Form<Value>) -> impl IntoResponse {
    log_link!("{res}");
    let ticket_size = res["ticket_size"].as_str().unwrap_or("0").parse::<usize>().unwrap_or(0);
    let mut vec_answer = vec![];
    for re in 1..ticket_size + 1 {
        let answer = res[format!("examine_{}", re)].as_str().unwrap_or("0").parse::<i32>().unwrap_or(0);
        log_info!("{answer}");
        vec_answer.push(answer)
    }
    log_info!("{vec_answer:?}");
    let check = check_examine(vec_answer, 1).await;
    let result = match check {
        true => "考试通过".to_string(),
        false => "考试不合格".to_string()
    };
    let template = http::ExamineCheckTemplate {
        title: "考试结果".to_string(),
        result,
    }.to_string();
    Html(template)
}

pub async fn examine_paper(Path(union_id): Path<i32>) -> impl IntoResponse {
    let template = http::PaperTemplate {
        title: "考卷列表".to_string(),
        papers: app::org_examine_paper::select_examine_paper_by_union(union_id).await.papers,
    }.to_string();
    Html(template)
}

pub async fn paper_insert() -> impl IntoResponse {
    let template = http::PaperInsertTemplate {
        title: "考卷新增".to_string(),
    }.to_string();
    Html(template)
}

pub async fn paper_update(Path(id): Path<i32>) -> impl IntoResponse {
    let template = http::PaperUpdateTemplate {
        title: "考卷更新".to_string(),
        paper: app::org_examine_paper::select_examine_paper_by_id(id).await,
    }.to_string();
    Html(template)
}

pub async fn public_setting() -> impl IntoResponse {
    let template = http::PublicSettingTemplate {
        title: "账户设置".to_string()
    }.to_string();
    Html(template)
}

pub async fn router(app_router: Router) -> Router {
    app_router
        .route("/", get(index))
        .route("/login", get(login))
        .route("/examine_client", get(examine_client))
        .route("/examine_update", get(examine_update))
        .route("/examine_check", post(examine_check))
        .route("/paper/:union_id", get(examine_paper))
        .route("/paper_insert", get(paper_insert))
        .route("/paper_update", post(paper_update))
        .route("/public_setting", get(public_setting))
}