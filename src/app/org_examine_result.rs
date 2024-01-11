use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use crate::app::get_pool;
use crate::{log_error, log_info, util};

pub type ExamineResults = Vec<ExamineResult>;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExamineResult {
    pub id: i64,
    pub user: String,
    pub union_id: i64,
    pub paper_id: i64,
    pub answers: Json<Vec<i64>>,
    pub result: bool,
    pub create_time: NaiveDateTime,
}

impl ExamineResult {
    pub fn update_to(user: String, union_id: i64, paper_id: i64, answers: Vec<i64>, result: bool) -> Self {
        Self {
            id: 0,
            user,
            union_id,
            paper_id,
            answers: Json(answers),
            result,
            create_time: Default::default(),
        }
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
            result: false,
            create_time: Default::default(),
        }
    }
}

pub async fn select_examine_results_by_union_id(union_id: i64) -> ExamineResults {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from org_examine_result where union_id = ?";
    let response = sqlx::query_as::<_, ExamineResult>(sql)
        .bind(union_id)
        .fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(_) => { ExamineResults::default() }
    };
    for re in res.clone() {
        let ans = re.answers.0;
        log_info!("{:?}",ans)
    }
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