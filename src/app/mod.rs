use std::str::FromStr;
use redis::{Connection, RedisResult};
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
pub mod org_examine_result;
pub mod redis_util;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server_port: i64,
    database_url: String,
    database_username: String,
    database_password: String,
    redis_url: String,
    redis_username: String,
    redis_password: String,
    redis_expire: i64,
}

#[derive(Clone, Copy, Serialize, Deserialize, sqlx::FromRow)]
pub struct TableCount {
    count: i64,
}

pub async fn get_pool() -> Result<Pool<MySql>, Error> {
    let config = get_config().await;
    let url_option = MySqlConnectOptions::from_str(&config.database_url)
        .expect("Database Link Error").username(&config.database_username).password(&get_config().await.database_password);
    sqlx::MySqlPool::connect_with(url_option).await
}

pub async fn get_config() -> Config {
    let mut yaml_file = tokio::fs::File::open("config.yaml").await.expect("read file error");
    let mut yaml_str = String::new();
    yaml_file.read_to_string(&mut yaml_str).await.expect("read str error");
    serde_yaml::from_str::<Config>(yaml_str.as_str()).expect("config error")
}

pub async fn get_redis_conn() -> RedisResult<Connection> {
    let config = get_config().await;
    let password = urlencoding::encode("Z#2nTt98ve!Q#*RY");
    let client = redis::Client::open(format!("redis://{}:{}@{}", config.redis_username, password, config.redis_url))?;
    let con = client.get_connection()?;
    Ok(con)
}