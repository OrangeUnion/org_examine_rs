use sqlx::types::chrono::NaiveDateTime;
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
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExamineAnswer {
    pub answer: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertExamine {
    pub problem: String,
    pub paper_id: i64,
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
            create_time: NaiveDateTime::default(),
            update_time: NaiveDateTime::default(),
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

pub async fn select_examine(id: i64) -> Examine {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from org_examine where id = ?";
    let response = sqlx::query_as::<_, Examine>(sql).bind(id).fetch_one(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(_) => { Examine::default() }
    };
    log_info!("{:?}",res.answers.0);
    res
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

pub async fn insert_examine(problem: String, paper_id: i64) -> u64 {
    let conn = get_pool().await.expect("Link Pool Error");
    let datetime = util::datatime::now_beijing_time();
    let default_answer = Json(vec!["默认考题，请删除本题并添加"]);
    let sql = "INSERT INTO org_examine (problem, answers, correct_answer, paper_id, create_time, update_time) VALUES (?, ?, ?, ?, ?, ?)";
    let response = sqlx::query(sql)
        .bind(problem)
        .bind(default_answer)
        .bind(1)
        .bind(paper_id)
        .bind(datetime)
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

pub async fn update_examine(update_examine: UpdateExamine) -> u64 {
    let conn = get_pool().await.expect("Link Pool Error");
    // answer json化
    let answer_value = Json(update_examine.answers);
    log_info!("json化 {answer_value:?}");
    let sql = "update org_examine set problem = ?, answers = ?, correct_answer = ?, problem_type = ?, paper_id = ?, update_time = ? where id = ?";
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

pub async fn delete_examine(id: i64) -> u64 {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "delete from org_examine where id = ?";
    let response = sqlx::query(sql)
        .bind(id)
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