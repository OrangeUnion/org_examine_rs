use redis::{Commands, RedisResult};
use serde::{Deserialize, Serialize};
use crate::app::{get_config, get_redis_conn};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedisUserInfo {
    pub(crate) userid: i64,
    pub(crate) username: String,
    pub(crate) role_urls: Vec<String>,
    pub(crate) union_id: i64,
    pub(crate) token: String,
}

impl RedisUserInfo {
    pub async fn redis_get_session(user: &str) -> RedisUserInfo {
        let mut con = get_redis_conn().await.expect("Redis链接失败");
        let key = format!("org_user_{}", user);
        let userid = con.hget::<_, &str, i64>(&key, "userid").unwrap_or(0);
        let username = con.hget::<_, &str, String>(&key, "username").unwrap_or("".to_string());
        let role_urls = con.hget::<_, &str, Vec<String>>(&key, "role_urls").unwrap_or(vec![]);
        let union_id = con.hget::<_, &str, i64>(&key, "union_id").unwrap_or(0);
        let token = con.hget::<_, &str, String>(&key, "token").unwrap_or("".to_string());
        Self {
            userid,
            username,
            role_urls,
            union_id,
            token,
        }
    }

    pub fn token_eq(&self, token: &str) -> bool {
        self.token.eq(token)
    }
}

pub async fn redis_save_session(redis_user_info: RedisUserInfo) -> RedisResult<bool> {
    let mut con = get_redis_conn().await.expect("Redis链接失败");
    redis::cmd("HMSET")
        .arg(format!("org_user_{}", redis_user_info.username))
        .arg("id")
        .arg(redis_user_info.userid)
        .arg("union_id")
        .arg(redis_user_info.union_id)
        .arg("token")
        .arg(redis_user_info.token)
        .query(&mut con)?;
    con.expire(format!("org_user_{}", redis_user_info.username), get_config().await.redis_expire)?;
    Ok(true)
}

pub async fn redis_start_examine(user: &str, union_id: i64, timeout: i64) -> RedisResult<()> {
    let mut con = get_redis_conn().await.expect("Redis链接失败");
    con.set(format!("start_examine_{union_id}_{user}"), timeout)?;
    con.expire(format!("start_examine_{union_id}_{user}"), timeout)?;
    Ok(())
}