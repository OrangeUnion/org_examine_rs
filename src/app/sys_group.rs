use std::fmt::Display;
use serde::{Deserialize, Serialize};
use sqlx::QueryBuilder;
use crate::app::get_pool;
use crate::{log_error, log_info};

pub type Groups = Vec<Group>;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Group {
    pub id: i64,
    pub title: String,
    pub name: String,
    pub status: i64,
}

impl Default for Group {
    fn default() -> Self {
        Self {
            id: 0,
            title: "".to_string(),
            name: "".to_string(),
            status: 0,
        }
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Group ([ID: {}], [Title: {}], [Name: {}], [Status: {}])", self.id, self.title, self.name, self.status)
    }
}

pub async fn select_all_groups() -> Groups {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from sys_group where id != 1";
    let response = sqlx::query_as::<_, Group>(sql)
        .fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(_) => { Groups::default() }
    };
    Groups::from(res)
}

pub async fn select_groups_by_user_id(userid: i64) -> (Vec<Group>, Vec<i64>) {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from sys_group a, sys_user_group b where a.id = b.group_id and b.user_id = ?";
    let response = sqlx::query_as::<_, Group>(sql)
        .bind(userid)
        .fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(_) => { Groups::default() }
    };
    let mut ids = vec![];
    for re in &res {
        ids.push(re.id)
    }
    (Groups::from(res), ids)
}

pub async fn select_user_groups(username: &str) -> Groups {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select b.* from sys_user a, sys_group b, sys_user_group c where a.username = ? and a.id = c.user_id and b.id = c.group_id";
    let response = sqlx::query_as::<_, Group>(sql)
        .bind(username)
        .fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(_) => { Groups::default() }
    };
    Groups::from(res)
}

pub async fn insert_group(userid: i64, group_ids: Vec<i64>) -> u64 {
    let conn = get_pool().await.expect("Link Pool Error");
    let mut sql = String::from("INSERT INTO sys_user_group (user_id, group_id) VALUES ");
    for group_id in group_ids {
        sql.push_str(&format!("({userid}, {group_id})"));
        sql.push(',')
    }
    let sql = sql.trim_end_matches(",");
    log_info!("{}", sql);
    let response = sqlx::query(sql).execute(&conn).await;
    let res = match response {
        Ok(r) => { r.rows_affected() }
        Err(e) => {
            log_error!("insert {e}");
            0
        }
    };
    log_info!("insert {res}");
    res
}

pub async fn delete_group(userid: i64) -> u64 {
    let conn = get_pool().await.expect("Link Pool Error");
    let response = QueryBuilder::new("delete from sys_user_group where user_id = ").push_bind(userid).build().execute(&conn).await;
    let res = match response {
        Ok(r) => { r.rows_affected() }
        Err(e) => {
            log_error!("delete {e}");
            0
        }
    };
    log_info!("delete {res}");
    res
}

pub async fn update_group(userid: i64, group_ids: Vec<i64>) -> u64 {
    let delete = delete_group(userid);
    let insert = insert_group(userid, group_ids);
    let res = tokio::join!(delete);
    let res2 = tokio::join!(insert);
    res.0 + res2.0
}