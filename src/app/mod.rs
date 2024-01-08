use std::str::FromStr;
use serde::{Deserialize, Serialize};
use sqlx::{Error, MySql, Pool};
use sqlx::mysql::MySqlConnectOptions;
use tokio::io::AsyncReadExt;

pub mod sys_user;
pub mod sys_group;
pub mod sys_role;
pub mod org_examine;
pub mod org_paper;
pub mod org_union;
pub mod org_examine_submit;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server_port: i32,
    database_url: String,
    database_username: String,
    database_password: String,
}

pub async fn get_pool() -> Result<Pool<MySql>, Error> {
    let url_option = MySqlConnectOptions::from_str(&get_config().await.database_url)
        .expect("Database Link Error").username(&get_config().await.database_username).password(&get_config().await.database_password);
    sqlx::MySqlPool::connect_with(url_option).await
}

pub async fn get_config() -> Config {
    let mut yaml_file = tokio::fs::File::open("config.yaml").await.expect("read file error");
    let mut yaml_str = String::new();
    yaml_file.read_to_string(&mut yaml_str).await.expect("read str error");
    serde_yaml::from_str::<Config>(yaml_str.as_str()).expect("config error")
}