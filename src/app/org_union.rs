use sqlx::types::chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use crate::app::get_pool;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unions {
    pub unions: Vec<Union>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Union {
    pub id: i32,
    pub name: String,
    pub status: i32,
    pub create_time: DateTime<Local>,
    pub update_time: DateTime<Local>,
}

impl Default for Union {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            status: 0,
            create_time: Local::now(),
            update_time: Local::now(),
        }
    }
}

impl Unions {
    fn from(vec_union: Vec<Union>) -> Self {
        Self {
            unions: vec_union,
        }
    }
}

pub async fn select_all_union() -> Unions {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from org_union";
    let response = sqlx::query_as::<_, Union>(sql)
        .fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(_) => { vec![Union::default()] }
    };
    Unions::from(res)
}