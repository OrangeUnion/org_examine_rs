use askama_axum::IntoResponse;
use axum::response::Html;
use axum::{Form, Router};
use axum::extract::Path;
use axum::routing::{get, post};
use serde_json::Value;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use crate::{http, log_info, log_link, util};
use crate::app::*;
use crate::controller::auths::*;

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

pub async fn users() -> impl IntoResponse {
    let template = http::UserTemplate {
        title: "用户列表".to_string(),
        users: sys_user::select_user_no_root().await,
    }.to_string();
    Html(template)
}

pub async fn insert_user() -> impl IntoResponse {
    let template = http::UserInsertTemplate {
        title: "新增用户".to_string(),
        groups: sys_group::select_all_groups().await,
        unions: org_union::select_all_union().await,
    }.to_string();
    Html(template)
}

pub async fn update_user(Path(id): Path<i64>) -> impl IntoResponse {
    let template = http::UserUpdateTemplate {
        title: "新增用户".to_string(),
        user: sys_user::select_user_by_id(id).await,
        groups: sys_group::select_all_groups().await,
        unions: org_union::select_all_union().await,
        user_groups_id: sys_group::select_groups_by_user_id(id).await.1,
    }.to_string();
    Html(template)
}

pub async fn examine_start() -> impl IntoResponse {
    let template = http::ExamineStartTemplate {
        title: "开始考试".to_string(),
        unions: org_union::select_all_union().await,
    }.to_string();
    Html(template)
}

pub async fn examine_client(Path((union_id, user, tag)): Path<(i64, String, String)>) -> impl IntoResponse {
    let paper_id = util::permission::random_paper_id_by_union(union_id).await;
    let examines = org_examine::select_examines_by_paper(paper_id).await;
    let template = http::ExamineClientTemplate {
        title: "考试题目".to_string(),
        examines,
        paper_id,
        union_id,
        user: user.clone(),
        tag: tag.clone(),
    }.to_string();
    redis_util::redis_start_examine(&user, union_id, 3600).await.expect("Start Failed");
    Html(template)
}

pub async fn examine_result(Path(union_id): Path<i64>) -> impl IntoResponse {
    let examine_results = match union_id {
        0 => org_examine_result::select_examine_results().await,
        _ => org_examine_result::select_examine_results_by_union_id(union_id).await,
    };
    let template = http::ExamineResultTemplate {
        title: "考试记录".to_string(),
        examine_results,
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

pub async fn examine_update(Path((id, problem_type)): Path<(i64, i64)>) -> impl IntoResponse {
    let examine = org_examine::select_examine(id).await;
    let template = match problem_type {
        1 => http::ExamineUpdateTemplate {
            title: "单选题配置".to_string(),
            examine: examine.clone(),
            correct_answer: examine.correct_answer.0,
        }.to_string(),
        2 => http::ExamineUpdate2Template {
            title: "多选题配置".to_string(),
            examine: examine.clone(),
            correct_answer: examine.correct_answer.0,
        }.to_string(),
        3 => http::ExamineUpdate3Template {
            title: "填空题配置".to_string(),
            examine: examine.clone(),
            correct_answer: examine.correct_answer.0,
        }.to_string(),
        4 => http::ExamineUpdate4Template {
            title: "问答题配置".to_string(),
            examine: examine.clone(),
            correct_answer: examine.correct_answer.0,
        }.to_string(),
        _ => "404".to_string()
    };
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
        .layer(AsyncRequireAuthorizationLayer::new(TokenAuth))
        .route("/", get(index))
        .route("/login", get(login).layer(AsyncRequireAuthorizationLayer::new(LoginAuth)))
        .route("/examine_start", get(examine_start))
        .route("/examine_client/:union_id/:user/:tag", get(examine_client))
}

pub async fn auth_router(app_router: Router) -> Router {
    app_router
        .route("/list_user", get(users))
        .route("/insert_user_view", get(insert_user))
        .route("/update_user_view/:id", get(update_user))
        .route("/list_paper_examines", post(list_paper_examines))
        .route("/examine_update/:id/:problem_type", get(examine_update))
        .route("/examine_results/:union_id", get(examine_result))
        .route("/paper/:union_id", get(papers))
        .route("/list_union", get(list_union))
        .route("/public_setting", get(public_setting))
}