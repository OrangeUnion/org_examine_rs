use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use serde_json::json;
use crate::{http, util};

pub async fn pwd() -> impl IntoResponse {
    let str = "1";
    let pwd = util::permission::encode_password(str);
    let ver = util::permission::verify_password(str, str);
    let ver_e = util::permission::verify_password(str, str);
    let json = json!({
        "str": str,
        "pwd": pwd,
        "ok": ver,
        "err": ver_e
    });
    (http::headers(), Json(json))
}

pub async fn router(app_router: Router) -> Router {
    app_router
        .route("/pwd", get(pwd))
}