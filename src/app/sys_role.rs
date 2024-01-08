use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::app::get_pool;

pub type Roles = Vec<Role>;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub status: i64
}

impl Default for Role {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            url: "".to_string(),
            status: 0,
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rule ([ID: {}], [Rule: {}], [Url: {}], [Status: {}])", self.id, self.name, self.url, self.status)
    }
}

pub async fn select_user_roles(username: &str) -> Roles {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select b.* from sys_user a, sys_role b, sys_user_group c, sys_role_group d where a.username = ? and a.id = c.user_id and b.id = d.role_id and c.group_id = d.group_id;";
    let response = sqlx::query_as::<_, Role>(sql)
        .bind(username)
        .fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(_) => { vec![Role::default()] }
    };
    Roles::from(res)
}