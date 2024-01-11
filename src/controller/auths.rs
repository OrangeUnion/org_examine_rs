use axum::body::Body;
use axum::http::{Request, Response, StatusCode};
use futures_util::future::BoxFuture;
use tower_http::auth::AsyncAuthorizeRequest;
use crate::{log_error, log_info, log_link, log_warn, util};
use crate::app::redis_util;

#[derive(Clone, Copy)]
pub struct TokenAuth;

#[derive(Clone, Copy)]
pub struct LoginAuth;

impl<B: Send + 'static> AsyncAuthorizeRequest<B> for TokenAuth {
    type RequestBody = B;
    type ResponseBody = Body;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, request: Request<B>) -> Self::Future {
        log_link!("{:?}", request.uri().query());
        Box::pin(async {
            let unauthorized_response = Response::builder()
                .status(StatusCode::FOUND)
                .header("Location", "/login") // 设置重定向的 URL
                .body(Body::empty())
                .unwrap();

            if let Some(uri_query) = request.uri().query() {
                let url = util::parse_query_string(uri_query);
                log_link!("{:?}", url);
                let user = url.get("user").unwrap_or(&"".to_string()).to_string();
                let token = url.get("token").unwrap_or(&"".to_string()).to_string();
                log_link!("{} {}", user, token);

                if !redis_util::RedisUserInfo::redis_get_session(&user).await.token_eq(&token) {
                    log_error!("not login");
                    return Err(unauthorized_response);
                };

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
        log_link!("{:?}", request.uri().query());
        Box::pin(async {
            let home_response = Response::builder()
                .status(StatusCode::FOUND)
                .header("Location", "/") // 设置重定向的 URL
                .body(Body::empty())
                .unwrap();

            if let Some(uri_query) = request.uri().query() {
                let url = util::parse_query_string(uri_query);
                log_link!("{:?}", url);
                let user = url.get("user").unwrap_or(&"".to_string()).to_string();
                let token = url.get("token").unwrap_or(&"".to_string()).to_string();
                log_link!("{} {}", user, token);

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