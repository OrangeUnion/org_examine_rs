use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::app::get_pool;

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

pub async fn select_user_groups(username: &str) -> Groups {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select b.* from sys_user a, sys_group b, sys_user_group c where a.username = ? and a.id = c.user_id and b.id = c.group_id";
    let response = sqlx::query_as::<_, Group>(sql)
        .bind(username)
        .fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(_) => { vec![Group::default()] }
    };
    Groups::from(res)
}