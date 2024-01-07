use askama_axum::IntoResponse;
use axum::response::Html;
use axum::{Form, Router};
use axum::routing::{get, post};
use serde_json::Value;
use crate::{app, http, log_info, log_link};
use crate::app::org_examination::check_examination;

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

pub async fn examination_client() -> impl IntoResponse {
    let template = http::ExaminationClientTemplate {
        title: "考试题目".to_string(),
        examinations: app::org_examination::union_examinations().await,
    }.to_string();
    Html(template)
}

pub async fn examination_update() -> impl IntoResponse {
    let template = http::ExaminationUpdateTemplate {
        title: "考题配置".to_string()
    }.to_string();
    Html(template)
}

pub async fn examination_check(Form(res): Form<Value>) -> impl IntoResponse {
    log_link!("{res}");
    let ticket = res["ticket"].as_str().unwrap_or("0").parse::<usize>().unwrap_or(0);
    let mut vec_answer = vec![];
    for re in 1..ticket {
        log_info!("{}",res[format!("examination{}",re)]);
        vec_answer.push(re as i32)
    }
    let check = check_examination(vec_answer, 1).await;
    let result = match check {
        true => "考试通过".to_string(),
        false => "考试不合格".to_string()
    };
    let template = http::ExaminationCheckTemplate {
        title: "考试结果".to_string(),
        result,
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
        .route("/examination_client", get(examination_client))
        .route("/examination_update", get(examination_update))
        .route("/examination_check", post(examination_check))
        .route("/public_setting", get(public_setting))
}