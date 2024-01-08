use sqlx::types::chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use crate::app::get_pool;
use crate::{log_error, log_info, util};

pub type Examines = Vec<Examine>;
pub type UpdateExamines = Vec<UpdateExamine>;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Examine {
    pub id: i64,
    pub problem: String,
    pub answers: Json<Vec<String>>,
    pub correct_answer: i64,
    pub problem_type: i64,
    pub paper_id: i64,
    pub status: i64,
    pub create_time: DateTime<Local>,
    pub update_time: DateTime<Local>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExamineAnswer {
    pub answer: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateExamine {
    pub id: i64,
    pub problem: String,
    pub answers: Vec<String>,
    pub correct_answer: i64,
    pub problem_type: i64,
    pub paper_id: i64,
}

impl Default for Examine {
    fn default() -> Self {
        Self {
            id: 0,
            problem: "".to_string(),
            answers: Json::default(),
            correct_answer: 0,
            problem_type: 0,
            paper_id: 0,
            status: 0,
            create_time: Local::now(),
            update_time: Local::now(),
        }
    }
}

impl Default for ExamineAnswer {
    fn default() -> Self {
        Self {
            answer: vec![],
        }
    }
}

pub async fn select_examines() -> Examines {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from org_examine";
    let response = sqlx::query_as::<_, Examine>(sql).fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(_) => { vec![Examine::default()] }
    };
    for re in res.clone() {
        let ans = re.answers.0;
        log_info!("{:?}",ans)
    }
    Examines::from(res)
}

pub async fn select_examines_by_paper(paper_id: i64) -> Examines {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from org_examine where paper_id = ?";
    let response = sqlx::query_as::<_, Examine>(sql).bind(paper_id).fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(_) => { vec![Examine::default()] }
    };
    for re in res.clone() {
        let ans = re.answers.0;
        log_info!("{:?}",ans)
    }
    Examines::from(res)
}

pub async fn select_update_examines_by_paper(paper_id: i64) -> UpdateExamines {
    let examines = select_examines_by_paper(paper_id).await;
    let mut vec_update_examine = UpdateExamines::new();
    for examine in examines {
        let ex = UpdateExamine {
            id: examine.id,
            problem: examine.problem,
            answers: examine.answers.0,
            correct_answer: examine.correct_answer,
            problem_type: examine.problem_type,
            paper_id: examine.paper_id,
        };
        vec_update_examine.push(ex)
    };
    UpdateExamines::from(vec_update_examine)
}

pub async fn update_examine(update_examine: UpdateExamine) -> u64 {
    let conn = get_pool().await.expect("Link Pool Error");
    let answer_value = serde_json::Value::from(update_examine.answers).to_string();
    log_info!("{answer_value}");
    let sql = "update org_examine set problem = ?, answers = ?, correct_answer = ?, problem_type = ?, org_union = ?, update_time = ? where id = ?";
    let response = sqlx::query(sql)
        .bind(update_examine.problem)
        .bind(answer_value)
        .bind(update_examine.correct_answer)
        .bind(update_examine.problem_type)
        .bind(update_examine.paper_id)
        .bind(util::datatime::now_beijing_time())
        .bind(update_examine.id)
        .execute(&conn).await;
    match response {
        Ok(r) => { r.rows_affected() }
        Err(e) => {
            log_error!("SQL Error {e}");
            0
        }
    }
}

pub async fn check_examine(post_answers: Vec<i64>, paper_id: i64) -> bool {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from org_examine where paper_id = ?";
    let response = sqlx::query_as::<_, Examine>(sql).bind(paper_id).fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(e) => {
            log_error!("SQL Error {e}");
            vec![Examine::default()]
        }
    };
    let mut correct_answers = Vec::new();
    for re in res {
        correct_answers.push(re.correct_answer)
    }
    log_info!("post: {post_answers:?} data: {correct_answers:?}");
    post_answers.eq(&correct_answers)
}