use askama::Template;
use axum::http::{header, HeaderName};
use axum::response::AppendHeaders;
use crate::app::org_examine::{Examine, Examines, ExamineValue};
use crate::app::org_examine_result::ExamineResults;
use crate::app::org_paper::Papers;
use crate::app::org_union::Unions;
use crate::app::sys_group::Groups;
use crate::app::sys_user::{User, Users};

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
#[template(path = "user/index.html")]
pub struct UserTemplate {
    pub title: String,
    pub users: Users,
}

#[derive(Template)]
#[template(path = "user/insert.html")]
pub struct UserInsertTemplate {
    pub title: String,
    pub groups: Groups,
    pub unions: Unions,
}

#[derive(Template)]
#[template(path = "user/update.html")]
pub struct UserUpdateTemplate {
    pub title: String,
    pub user: User,
    pub groups: Groups,
    pub unions: Unions,
    pub user_groups_id: Vec<i64>,
}

#[derive(Template)]
#[template(path = "examine/start.html")]
pub struct ExamineStartTemplate {
    pub title: String,
    pub unions: Unions,
}

#[derive(Template)]
#[template(path = "examine/client.html")]
pub struct ExamineClientTemplate {
    pub title: String,
    pub examines: Examines,
    pub paper_id: i64,
    pub union_id: i64,
    pub user: String,
    pub tag: String,
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
    pub examine: Examine,
    pub correct_answer: ExamineValue,
}

#[derive(Template)]
#[template(path = "examine/update2.html")]
pub struct ExamineUpdate2Template {
    pub title: String,
    pub examine: Examine,
    pub correct_answer: ExamineValue,
}

#[derive(Template)]
#[template(path = "examine/update3.html")]
pub struct ExamineUpdate3Template {
    pub title: String,
    pub examine: Examine,
    pub correct_answer: ExamineValue,
}

#[derive(Template)]
#[template(path = "examine/update4.html")]
pub struct ExamineUpdate4Template {
    pub title: String,
    pub examine: Examine,
    pub correct_answer: ExamineValue,
}

#[derive(Template)]
#[template(path = "examine/result.html")]
pub struct ExamineResultTemplate {
    pub title: String,
    pub examine_results: ExamineResults,
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
#[template(path = "union/index.html")]
pub struct UnionTemplate {
    pub title: String,
    pub unions: Unions,
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