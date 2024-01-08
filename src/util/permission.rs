use serde_json::{json, Value};
use crate::app;

pub fn encode_password(password: &str) -> String {
    let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST);
    hash.expect("Password Error")
}

pub fn verify_password(password: &str, data_password: &str) -> bool {
    let verify = bcrypt::verify(password, data_password);
    verify.expect("is Err ")
}

pub async fn list_user_groups(username: &str) -> (Vec<String>, Vec<String>) {
    let groups = app::sys_group::select_user_groups(username).await;
    let (mut titles, mut names) = (Vec::new(), Vec::new());
    let _ = groups.iter().map(|group| {
        let group = group.clone();
        titles.push(group.title);
        names.push(group.name);
        0
    }).collect::<Vec<_>>();
    (titles, names)
}

pub async fn list_user_roles(username: &str) -> (Vec<String>, Vec<String>) {
    let roles = app::sys_role::select_user_roles(username).await;
    let (mut names, mut urls) = (Vec::new(), Vec::new());
    let _ = roles.iter().map(|role| {
        let role = role.clone();
        names.push(role.name);
        urls.push(role.url);
        0
    }).collect::<Vec<_>>();
    (names, urls)
}

pub async fn list_user(username: &str) -> Value {
    let (_, user_groups) = list_user_groups(username).await;
    let (_, user_roles) = list_user_roles(username).await;
    let value = json!({
        "username": username,
        "user_groups": user_groups,
        "user_roles": user_roles
    });
    value
}

pub async fn show_user(username: &str) -> Value {
    let (user_groups, _) = list_user_groups(username).await;
    let (user_roles, _) = list_user_roles(username).await;
    let value = json!({
        "username": username,
        "user_groups": user_groups,
        "user_roles": user_roles
    });
    value
}