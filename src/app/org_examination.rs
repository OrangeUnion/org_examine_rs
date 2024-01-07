use std::ops::Add;
use sqlx::types::chrono::{DateTime, FixedOffset, Local};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use crate::app::get_pool;
use crate::{log_error, log_info};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Examinations {
    pub examinations: Vec<Examination>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Examination {
    pub id: i32,
    pub problem: String,
    pub answers: Json<Vec<String>>,
    pub correct_answer: i32,
    pub problem_type: i32,
    pub org_union: i32,
    pub status: i32,
    pub create_time: DateTime<Local>,
    pub update_time: DateTime<Local>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExaminationAnswer {
    pub answer: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateExamination {
    pub id: i32,
    pub problem: String,
    pub answers: Vec<String>,
    pub correct_answer: i32,
    pub problem_type: i32,
    pub org_union: i32,
}

impl Default for Examination {
    fn default() -> Self {
        Self {
            id: 0,
            problem: "".to_string(),
            answers: Json::default(),
            correct_answer: 0,
            problem_type: 0,
            org_union: 0,
            status: 0,
            create_time: Local::now(),
            update_time: Local::now(),
        }
    }
}

impl Default for ExaminationAnswer {
    fn default() -> Self {
        Self {
            answer: vec![],
        }
    }
}

impl Examinations {
    fn from(vec_examination: Vec<Examination>) -> Self {
        Self {
            examinations: vec_examination,
        }
    }
}

pub async fn select_examinations() -> Examinations {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from org_examination";
    let response = sqlx::query_as::<_, Examination>(sql).fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(_) => { vec![Examination::default()] }
    };
    for re in res.clone() {
        let ans = re.answers.0;
        log_info!("{:?}",ans)
    }
    Examinations::from(res)
}

pub async fn union_examinations() -> Vec<UpdateExamination> {
    let examinations = select_examinations().await.examinations;
    let mut vec_update_examination = vec![];
    for examination in examinations {
        let ex = UpdateExamination {
            id: examination.id,
            problem: examination.problem,
            answers: examination.answers.0,
            correct_answer: examination.correct_answer,
            problem_type: examination.problem_type,
            org_union: examination.org_union,
        };
        vec_update_examination.push(ex)
    };
    vec_update_examination
}

pub async fn update_examination(update_examination: UpdateExamination) -> u64 {
    let conn = get_pool().await.expect("Link Pool Error");
    let answer_value = serde_json::Value::from(update_examination.answers).to_string();
    let dt = FixedOffset::east_opt(8 * 3600).unwrap();
    let beijing_time = DateTime::add(Local::now(), dt);
    log_info!("{answer_value}");
    let sql = "update org_examination set problem = ?, answers = ?, correct_answer = ?, problem_type = ?, org_union = ?, update_time = ? where id = ?";
    let response = sqlx::query(sql)
        .bind(update_examination.problem)
        .bind(answer_value)
        .bind(update_examination.correct_answer)
        .bind(update_examination.problem_type)
        .bind(update_examination.org_union)
        .bind(beijing_time)
        .bind(update_examination.id)
        .execute(&conn).await;
    match response {
        Ok(r) => { r.rows_affected() }
        Err(e) => {
            log_error!("SQL Error {e}");
            0
        }
    }
}

pub async fn check_examination(post_answers: Vec<i32>, org_union: i32) -> bool {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from org_examination where org_union = ?";
    let response = sqlx::query_as::<_, Examination>(sql).bind(org_union).fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(e) => {
            log_error!("SQL Error {e}");
            vec![Examination::default()]
        }
    };
    let mut correct_answers = Vec::new();
    for re in res {
        correct_answers.push(re.correct_answer)
    }
    log_info!("post: {post_answers:?} data: {correct_answers:?}");
    post_answers.eq(&correct_answers)
}