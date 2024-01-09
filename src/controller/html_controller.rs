use askama_axum::IntoResponse;
use axum::response::Html;
use axum::{Form, Router};
use axum::extract::Path;
use axum::routing::{get, post};
use rand::Rng;
use serde_json::Value;
use crate::{http, log_info, log_link};
use crate::app::org_examine::check_examine;
use crate::app::*;

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

pub async fn examine_start() -> impl IntoResponse {
    let template = http::ExamineStartTemplate {
        title: "开始考试".to_string(),
    }.to_string();
    Html(template)
}

pub async fn examine_client(Path((union_id, user)): Path<(i64, String)>) -> impl IntoResponse {
    let union_papers = org_paper::select_papers_by_union(union_id).await;
    let random_paper = rand::thread_rng().gen_range(1..=union_papers.len()) as i64;
    let template = http::ExamineClientTemplate {
        title: "考试题目".to_string(),
        examines: org_examine::select_update_examines_by_paper(random_paper).await,
        paper_id: random_paper,
        user,
    }.to_string();
    Html(template)
}

pub async fn list_paper_examines(Form(res): Form<Value>) -> impl IntoResponse {
    log_link!("{res}");
    let paper_id = res["paper_id"].as_str().unwrap_or("0").parse().unwrap_or(0);
    let paper_title = res["paper_title"].as_str().unwrap_or("");
    let examines = match paper_id {
        0 => org_examine::select_examines().await,
        _ => org_examine::select_examines_by_paper(paper_id).await,
    };
    let template = http::ExaminePaperTemplate {
        title: paper_title.to_string(),
        paper_id,
        examines,
    }.to_string();
    Html(template)
}

pub async fn examine_update(Path(id): Path<i64>) -> impl IntoResponse {
    let examine = org_examine::select_examine(id).await;
    let template = http::ExamineUpdateTemplate {
        title: "考题配置".to_string(),
        examine: examine.clone(),
        correct_answer: examine.correct_answer as usize,
    }.to_string();
    Html(template)
}

pub async fn examine_check(Form(res): Form<Value>) -> impl IntoResponse {
    log_link!("{res}");
    let ticket_size = res["ticket_size"].as_str().unwrap_or("0").parse::<usize>().unwrap_or(0);
    let mut vec_answer = vec![];
    for re in 1..ticket_size + 1 {
        let answer = res[format!("examine_{}", re)].as_str().unwrap_or("0").parse::<i64>().unwrap_or(0);
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

pub async fn papers(Path(union_id): Path<i64>) -> impl IntoResponse {
    let papers = match union_id {
        0 => org_paper::select_papers().await,
        _ => org_paper::select_papers_by_union(union_id).await,
    };
    let template = http::PaperTemplate {
        title: "考卷列表".to_string(),
        papers,
    }.to_string();
    Html(template)
}

pub async fn paper_insert() -> impl IntoResponse {
    let template = http::PaperInsertTemplate {
        title: "考卷新增".to_string(),
    }.to_string();
    Html(template)
}

pub async fn paper_update(Path(id): Path<i64>) -> impl IntoResponse {
    let template = http::PaperUpdateTemplate {
        title: "考卷更新".to_string(),
        paper: org_paper::select_paper_by_id(id).await,
    }.to_string();
    Html(template)
}

pub async fn public_setting() -> impl IntoResponse {
    let template = http::PublicSettingTemplate {
        title: "账户设置".to_string()
    }.to_string();
    Html(template)
}

pub async fn list_union() -> impl IntoResponse {
    let template = http::UnionTemplate {
        title: "联盟列表".to_string(),
        unions: org_union::select_all_union().await,
    }.to_string();
    Html(template)
}

pub async fn router(app_router: Router) -> Router {
    app_router
        .route("/", get(index))
        .route("/login", get(login))
        .route("/examine_start", get(examine_start))
        .route("/examine_client/:union_id/:user", get(examine_client))
        .route("/list_paper_examines", post(list_paper_examines))
        .route("/examine_update/:id", get(examine_update))
        .route("/examine_check", post(examine_check))
        .route("/paper/:union_id", get(papers))
        .route("/paper_insert", get(paper_insert))
        .route("/paper_update", post(paper_update))
        .route("/list_union", get(list_union))
        .route("/public_setting", get(public_setting))
}