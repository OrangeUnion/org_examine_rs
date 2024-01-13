use sqlx::types::chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use crate::app::{get_pool, org_examine_result};
use crate::{log_error, log_info, util};
use crate::app::org_examine_result::{CheckResult, UpdateCheck};

pub type Examines = Vec<Examine>;
pub type ExamineValue = Vec<String>;
pub type ExamineValues = Vec<ExamineValue>;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Examine {
    pub id: i64,
    pub problem: String,
    pub answers: Json<ExamineValue>,
    pub correct_answer: Json<ExamineValue>,
    pub problem_type: i64,
    pub paper_id: i64,
    pub status: i64,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertExamine {
    pub problem: String,
    pub problem_type: i64,
    pub paper_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateExamine {
    pub id: i64,
    pub problem: String,
    pub answers: ExamineValue,
    pub correct_answer: ExamineValue,
    pub problem_type: i64,
    pub paper_id: i64,
}

impl Default for Examine {
    fn default() -> Self {
        Self {
            id: 0,
            problem: "".to_string(),
            answers: Json::default(),
            correct_answer: Json::default(),
            problem_type: 0,
            paper_id: 0,
            status: 0,
            create_time: NaiveDateTime::default(),
            update_time: NaiveDateTime::default(),
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
        Err(e) => {
            log_error!("{e}");
            vec![Examine::default()]
        }
    };
    for re in res.clone() {
        let ans = re.answers.0;
        log_info!("{:?}",ans)
    }
    Examines::from(res)
}

pub async fn insert_examine(insert_examine: InsertExamine) -> u64 {
    let conn = get_pool().await.expect("Link Pool Error");
    let datetime = util::datetime::now_beijing_time();
    let default_answer = Json(vec!["默认答案，请删除本答案并添加"]);
    let default_correct_answer = Json(ExamineValue::from(["1".to_string()]));
    let sql = "INSERT INTO org_examine (problem, answers, correct_answer, problem_type, paper_id, create_time, update_time) VALUES (?, ?, ?, ?, ?, ?, ?)";
    let response = sqlx::query(sql)
        .bind(insert_examine.problem)
        .bind(default_answer)
        .bind(default_correct_answer)
        .bind(insert_examine.problem_type)
        .bind(insert_examine.paper_id)
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
    // 答案json化
    let answer_value = Json(update_examine.answers);
    let correct_answer_value = Json(update_examine.correct_answer);
    log_info!("json化 {answer_value:?}");
    let sql = "update org_examine set problem = ?, answers = ?, correct_answer = ?, problem_type = ?, paper_id = ?, update_time = ? where id = ?";
    let response = sqlx::query(sql)
        .bind(update_examine.problem)
        .bind(answer_value)
        .bind(correct_answer_value)
        .bind(update_examine.problem_type)
        .bind(update_examine.paper_id)
        .bind(util::datetime::now_beijing_time())
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

pub async fn check_examine(update_check: UpdateCheck) -> CheckResult {
    // 查询用户提交次数
    let user_examine_count = org_examine_result::count_examine_results_by_user(&update_check.user).await;
    log_info!("COUNT {}考试次数{user_examine_count}",&update_check.user);
    // 查询考卷下的题目
    let res = select_examines_by_paper(update_check.paper_id).await;
    // 打包正确答案
    let mut correct_answers = ExamineValues::new();
    for re in res {
        correct_answers.push(re.correct_answer.0)
    }
    log_info!("提交答案: {:?} 正确答案: {correct_answers:?}", update_check.answers);

    if user_examine_count > 10 {
        return CheckResult::Overrun;
    };
    let result = match update_check.answers.eq(&correct_answers) {
        true => CheckResult::Pass,
        false => CheckResult::UnPass,
    };

    // 写入数据库
    let examine_result = org_examine_result::ExamineResult::update_to(update_check, result);
    let to_result = org_examine_result::insert_examine_results(examine_result).await;
    log_info!("RESULT写入 {to_result}");
    result
}