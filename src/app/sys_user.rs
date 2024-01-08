use std::fmt::Display;
use std::vec;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::types::chrono::{DateTime, Local};
use crate::app::get_pool;
use crate::{log_error, log_info, log_link, log_warn};
use crate::util::permission;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Users {
    pub users: Vec<User>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub status: i32,
    pub r#type: i32,
    pub expire_time: DateTime<Local>,
    pub create_time: DateTime<Local>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            username: "".to_string(),
            password: "".to_string(),
            status: 0,
            r#type: 0,
            expire_time: DateTime::default(),
            create_time: Local::now(),
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User ([ID: {}], [Username: {}], [Password: {}], [Status: {}], [Type: {}])", self.id, self.username, self.password, self.status, self.r#type)
    }
}

impl Users {
    fn from(vec_user: Vec<User>) -> Self {
        Self {
            users: vec_user,
        }
    }
}

pub async fn select_all_user() -> Users {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from sys_user";
    let response = sqlx::query_as::<_, User>(sql).fetch_all(&conn).await;
    let res = match response {
        Ok(r) => { r }
        Err(_) => { vec![User::default()] }
    };
    Users::from(res)
}

pub async fn select_one_user(username: &str) -> User {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from sys_user where username = ?";
    let response = sqlx::query_as::<_, User>(sql).bind(username).fetch_one(&conn).await;
    match response {
        Ok(r) => { r }
        Err(_) => { User::default() }
    }
}

pub async fn _has_user(username: &str) -> bool {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from sys_user where username = ?";
    let response = sqlx::query_as::<_, User>(sql).bind(username).fetch_one(&conn).await;
    match response {
        Ok(_) => { true }
        Err(_) => { false }
    }
}

pub async fn check_login(username: &str, password: &str) -> Value {
    let conn = get_pool().await.expect("Link Pool Error");
    let sql = "select * from sys_user where username = ? and status = 1";
    let response = sqlx::query_as::<_, User>(sql)
        .bind(username)
        .fetch_one(&conn).await;
    match response {
        Ok(res) => {
            log_info!("input {password}");
            log_info!("data {}", res.password);
            let is_login = permission::verify_password(password, res.password.as_str());
            if !is_login {
                log_warn!("Login Failed");
                return json!({
                    "mag": "Login Failed",
                    "is_login": false
                })
            };
            let (_role_names, role_urls) = permission::list_user_roles(username).await;
            let token = permission::encode_password(format!("{}{:?}", username, role_urls).as_str());
            log_link!("Login Successful");
            json!({
                "mag": "Login Successful",
                "userid": res.id,
                "username": res.username,
                "is_login": true,
                "roles": role_urls,
                "token": token
            })
        }
        Err(e) => {
            log_error!("Login Error {e}");
            json!({
                "mag": "Login Error",
                "is_login": false,
            })
        }
    }
}