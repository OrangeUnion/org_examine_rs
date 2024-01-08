use askama::Template;
use axum::http::{header, HeaderName};
use axum::response::AppendHeaders;
use crate::app::org_examine::{Examines, UpdateExamines};
use crate::app::org_paper::{Paper, Papers};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "examine/start.html")]
pub struct ExamineStartTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "examine/client.html")]
pub struct ExamineClientTemplate {
    pub title: String,
    pub examines: UpdateExamines,
    pub paper_id: i64,
    pub user: String,
}

#[derive(Template)]
#[template(path = "examine/paper.html")]
pub struct ExaminePaperTemplate {
    pub title: String,
    pub paper_id: i64,
    pub examines: Examines,
}

#[derive(Template)]
#[template(path = "examine/update.html")]
pub struct ExamineUpdateTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "examine/check.html")]
pub struct ExamineCheckTemplate {
    pub title: String,
    pub result: String,
}

#[derive(Template)]
#[template(path = "paper/index.html")]
pub struct PaperTemplate {
    pub title: String,
    pub papers: Papers,
}

#[derive(Template)]
#[template(path = "paper/insert.html")]
pub struct PaperInsertTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "paper/update.html")]
pub struct PaperUpdateTemplate {
    pub title: String,
    pub paper: Paper,
}

#[derive(Template)]
#[template(path = "public/setting.html")]
pub struct PublicSettingTemplate {
    pub title: String,
}

pub fn headers() -> AppendHeaders<[(HeaderName, &'static str); 4]> {
    let headers = AppendHeaders([
        (header::CONTENT_TYPE, "application/json"),
        (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
        (header::ACCESS_CONTROL_ALLOW_METHODS, "POST, GET, OPTIONS, DELETE"),
        (header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"),
    ]);
    headers
}