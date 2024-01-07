use std::str::FromStr;
use sqlx::{Error, MySql, Pool};
use sqlx::mysql::MySqlConnectOptions;

pub mod sys_user;
pub mod sys_group;
pub mod sys_role;
pub mod org_examination;

pub async fn get_pool() -> Result<Pool<MySql>, Error> {
    let url_option = MySqlConnectOptions::from_str("mariadb://localhost:3306/org_system")
        .expect("Database Link Error").username("root").password("369");
    sqlx::MySqlPool::connect_with(url_option).await
}