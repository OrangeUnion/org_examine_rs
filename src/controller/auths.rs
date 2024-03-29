use axum::body::Body;
use axum::http::{Request, Response, StatusCode};
use futures_util::future::BoxFuture;
use tower_http::auth::AsyncAuthorizeRequest;
use crate::{log_error, log_info, log_link, log_warn, util};
use crate::app::redis_util;
use crate::app::sys_role;

#[derive(Clone, Copy)]
pub struct TokenAuth;

#[derive(Clone, Copy)]
pub struct LoginAuth;

impl<B: Send + 'static> AsyncAuthorizeRequest<B> for TokenAuth {
    type RequestBody = B;
    type ResponseBody = Body;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, request: Request<B>) -> Self::Future {
        Box::pin(async {
            let unauthorized_response = Response::builder()
                .status(StatusCode::FOUND)
                .header("Location", "/login") // 设置重定向的 URL
                .body(Body::empty())
                .unwrap();
            let unauthorized_to_home = Response::builder()
                .status(StatusCode::FOUND)
                .header("Location", "/") // 设置重定向的 URL
                .body(Body::empty())
                .unwrap();

            if let Some(uri_query) = request.uri().query() {
                let url = util::parse_query_string(uri_query);
                let user = url.get("user").unwrap_or(&"".to_string()).to_string();
                let token = url.get("token").unwrap_or(&"".to_string()).to_string();
                let uri_path = request.uri().path();
                log_info!("URI {uri_path}");

                let user_roles = sys_role::select_user_roles(&user).await;
                let mut check_uri_role = 0;
                for user_role in user_roles {
                    log_link!("Role {check_uri_role}");
                    if uri_path.contains(&user_role.url) || "all".eq(&user_role.url) {
                        check_uri_role += 1;
                    };
                };
                log_info!("check_uri_role {check_uri_role}");

                if !redis_util::RedisUserInfo::redis_get_session(&user).await.token_eq(&token) {
                    log_warn!("not login");
                    return Err(unauthorized_response);
                };
                if check_uri_role < 1 {
                    log_error!("Unauthorized");
                    return Err(unauthorized_to_home);
                }

                log_info!("is login");
                Ok(request)
            } else {
                log_error!("not login session");
                Err(unauthorized_response)
            }
        })
    }
}

impl<B: Send + 'static> AsyncAuthorizeRequest<B> for LoginAuth {
    type RequestBody = B;
    type ResponseBody = Body;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, request: Request<B>) -> Self::Future {
        Box::pin(async {
            let home_response = Response::builder()
                .status(StatusCode::FOUND)
                .header("Location", "/") // 设置重定向的 URL
                .body(Body::empty())
                .unwrap();

            if let Some(uri_query) = request.uri().query() {
                let url = util::parse_query_string(uri_query);
                let user = url.get("user").unwrap_or(&"".to_string()).to_string();
                let token = url.get("token").unwrap_or(&"".to_string()).to_string();

                if redis_util::RedisUserInfo::redis_get_session(&user).await.token_eq(&token) {
                    log_warn!("is login");
                    return Err(home_response);
                }

                log_info!("to login");
                Ok(request)
            } else {
                log_info!("to login session");
                Ok(request)
            }
        })
    }
}