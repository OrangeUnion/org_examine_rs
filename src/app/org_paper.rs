use sqlx::types::chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use crate::app::get_pool;
use crate::{log_error, util};

pub type Papers = Vec<Paper>;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Paper {
    pub id: i64,
    pub title: String,
    pub status: i64,
    pub union_id: i64,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePaper {
    pub id: i64,
    pub title: String,
    pub union_id: i64,
}

impl Default for Paper {
    fn default() -> Self {
        Self {
            id: 0,
            title: "".to_string(),
            status: 0,
            union_id: 0,
            create_time: NaiveDateTime::default(),
            update_time: NaiveDateTime::default(),
        }
    }
}

pub async fn select_papers() -> Papers {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from org_paper";
    let response = sqlx::query_as::<_, Paper>(sql)
        .fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(_) => { vec![Paper::default()] }
    };
    Papers::from(res)
}

pub async fn select_papers_by_union(union_id: i64) -> Papers {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from org_paper where union_id = ?";
    let response = sqlx::query_as::<_, Paper>(sql)
        .bind(union_id)
        .fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(_) => { vec![Paper::default()] }
    };
    Papers::from(res)
}

pub async fn select_paper_by_id(id: i64) -> Paper {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from org_paper where id = ?";
    let response = sqlx::query_as::<_, Paper>(sql)
        .bind(id)
        .fetch_one(&conn).await;
    match response {
        Ok(r) => { r }
        Err(_) => { Paper::default() }
    }
}

pub async fn update_paper_status(id: i64, mut status: i64) -> u64 {
    // 判断当前status状态，决定修改的状态
    status = match status {
        1 => 0,
        _ => 1
    };
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "update org_paper set status = ? where id = ?";
    let response = sqlx::query(sql)
        .bind(status).bind(id)
        .execute(&conn).await;
    match response {
        Ok(r) => { r.rows_affected() }
        Err(e) => {
            log_error!("SQL Error {e}");
            0
        }
    }
}

pub async fn insert_paper(title: &str, union_id: i64) -> u64 {
    let conn = get_pool().await.expect("Link Pool Error");
    let datetime = util::datetime::now_beijing_time();
    let sql = "INSERT INTO org_paper (title, union_id, create_time, update_time) VALUES (?, ?, ?, ?)";
    let response = sqlx::query(sql)
        .bind(title)
        .bind(union_id)
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

pub async fn update_paper(id: i64, title: &str, union_id: i64) -> u64 {
    let title_update = match title {
        "" => { "".to_string() }
        _ => { format!("title = '{}', ", title) }
    };
    let union_id_update = match union_id {
        0 => { "".to_string() }
        _ => { format!("union_id = {}, ", union_id) }
    };
    let conn = get_pool().await.expect("Link Pool Error");
    let datetime = util::datetime::now_beijing_time();
    let sql = format!("update org_paper set {title_update}{union_id_update}update_time = ? where id = ?");
    let response = sqlx::query(&sql)
        .bind(datetime)
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

pub async fn delete_paper(id: i64) -> u64 {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "delete from org_paper where id = ?";
    let delete_examine_sql = "delete from org_examine where paper_id = ?";
    let response = sqlx::query(&sql).bind(id).execute(&conn).await;
    let response_examine = sqlx::query(&delete_examine_sql).bind(id).execute(&conn).await;
    let res = match response {
        Ok(r) => { r.rows_affected() }
        Err(e) => {
            log_error!("SQL Error {e}");
            0
        }
    };
    let res_ex = match response_examine {
        Ok(r) => { r.rows_affected() }
        Err(e) => {
            log_error!("SQL Error {e}");
            0
        }
    };
    res + res_ex
}