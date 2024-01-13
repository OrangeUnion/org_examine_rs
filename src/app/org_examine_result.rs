use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use crate::app::{get_pool, TableCount};
use crate::{log_error, util};
use crate::app::org_examine::ExamineValues;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type, Copy)]
#[repr(i8)]
pub enum CheckResult {
    Pass,
    UnPass,
    Overrun,
    TimeOut,
    None,
}

pub type ExamineResults = Vec<ExamineResult>;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExamineResult {
    pub id: i64,
    pub user: String,
    pub union_id: i64,
    pub paper_id: i64,
    pub answers: Json<ExamineValues>,
    pub result: CheckResult,
    pub create_time: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateCheck {
    pub user: String,
    pub union_id: i64,
    pub paper_id: i64,
    pub ticket_size: i64,
    pub answers: ExamineValues,
}

impl ExamineResult {
    pub fn update_to(update_check: UpdateCheck, result: CheckResult) -> Self {
        Self {
            id: 0,
            user: update_check.user,
            union_id: update_check.union_id,
            paper_id: update_check.paper_id,
            answers: Json(update_check.answers),
            result,
            create_time: Default::default(),
        }
    }
}

impl Default for CheckResult {
    fn default() -> Self {
        Self::None
    }
}

impl Default for ExamineResult {
    fn default() -> Self {
        Self {
            id: 0,
            user: "".to_string(),
            union_id: 0,
            paper_id: 0,
            answers: Default::default(),
            result: Default::default(),
            create_time: Default::default(),
        }
    }
}

pub async fn select_examine_results() -> ExamineResults {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from org_examine_result";
    let response = sqlx::query_as::<_, ExamineResult>(sql)
        .fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(e) => {
            log_error!("{e}");
            ExamineResults::default()
        }
    };
    ExamineResults::from(res)
}

pub async fn select_examine_results_by_user(user: &str) -> ExamineResults {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from org_examine_result where user = ?";
    let response = sqlx::query_as::<_, ExamineResult>(sql)
        .bind(user)
        .fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(e) => {
            log_error!("{e}");
            ExamineResults::default()
        }
    };
    ExamineResults::from(res)
}

pub async fn count_examine_results_by_user(user: &str) -> i64 {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select count(id) as count from org_examine_result where user = ?";
    let response = sqlx::query_as::<_, TableCount>(sql)
        .bind(user)
        .fetch_one(&conn).await;
    let res = match response {
        Ok(r) => { r.count }
        Err(e) => {
            log_error!("{e}");
            -1
        }
    };
    res
}

pub async fn select_examine_results_by_union_id(union_id: i64) -> ExamineResults {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from org_examine_result where union_id = ?";
    let response = sqlx::query_as::<_, ExamineResult>(sql)
        .bind(union_id)
        .fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(e) => {
            log_error!("{e}");
            ExamineResults::default()
        }
    };
    ExamineResults::from(res)
}

pub async fn insert_examine_results(examine_result: ExamineResult) -> u64 {
    let conn = get_pool().await.expect("Link Pool Error");
    let datetime = util::datetime::now_beijing_time();
    let sql = "INSERT INTO org_examine_result (user, union_id, paper_id, answers, result, create_time) VALUES (?, ?, ?, ?, ?, ?)";
    let response = sqlx::query(sql)
        .bind(examine_result.user)
        .bind(examine_result.union_id)
        .bind(examine_result.paper_id)
        .bind(examine_result.answers)
        .bind(examine_result.result)
        .bind(datetime)
        .execute(&conn).await;
    match response {
        Ok(r) => { r.rows_affected() }
        Err(e) => {
            log_error!("SQL Error {e}");
            0
        }
    }
}