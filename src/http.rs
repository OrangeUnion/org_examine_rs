use askama::Template;
use axum::http::{header, HeaderName};
use axum::response::AppendHeaders;
use crate::app::org_examination::UpdateExamination;

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
#[template(path = "examination/client.html")]
pub struct ExaminationClientTemplate {
    pub title: String,
    pub examinations: Vec<UpdateExamination>,
}

#[derive(Template)]
#[template(path = "examination/update.html")]
pub struct ExaminationUpdateTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "examination/check.html")]
pub struct ExaminationCheckTemplate {
    pub title: String,
    pub result: String,
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