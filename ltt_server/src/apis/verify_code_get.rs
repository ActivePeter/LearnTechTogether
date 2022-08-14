// 验证码是发到邮箱里的，根据随机函数随机生成一个六位整数
// 然后将这样一个pair加入到一个哈希表中
// 最后就是定时功能，超过一定时间这个验证码就失效。

use axum::{Json, http};
use axum::response::IntoResponse;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::services::token;
use crate::services::verifycode::G_VERIFY_MANAGER;
use crate::services::email_service::{G_EMAIL_MANAGER, EmailSendResult};
use crate::models::user::UserId;
use axum::body::{HttpBody, Body};
use axum::extract::ConnectInfo;
use std::net::SocketAddr;
use std::future::Future;


pub async fn verify_code_get(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Json<RequestContent>,
) -> impl IntoResponse {
    println!("addr{}",addr);
    let v_=G_VERIFY_MANAGER.get_code(&req.email).await;
    if let Some(v)=v_{
        return match G_EMAIL_MANAGER.send_verify_code(&*req.email, v) {
            EmailSendResult::EmailSendFail => {
                (StatusCode::BAD_REQUEST, "sendfail").into_response()
            }
            EmailSendResult::Succ => {
                (StatusCode::OK, "sent").into_response()
            }
            EmailSendResult::EmailParseFail => {
                (StatusCode::BAD_REQUEST, "parsefail").into_response()
            }
        }
    }
    return (StatusCode::BAD_REQUEST, "wait").into_response();
}

// the input to our `create_user` handler
#[derive(Deserialize, Serialize)]
pub struct RequestContent {
    pub email:String
}

