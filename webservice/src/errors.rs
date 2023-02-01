// 创建自定义错误处理器

// 1. 创建一个自定义错误类型
// 2. 实现 From trait, 用于把其他错误类型转化为该类型
// 3. 为自定义错误类型实现 ResponseError trait
// 4. 在 handler 里返回自定义错误类型
// 5. Actix 会把错误转换为 HTTP 响应

use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

// 默认为 MyError 实现 Debug 和 Serialize
#[derive(Debug, Serialize)]
// 自定义错误类型
pub enum MyError {
    // 下面三个变体都可以存一个字符串
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String), // 前端非法传递
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    // 传递给用户的错误响应内容, 包含具体的错误消息
    error_message: String,
}

// 为 MyError实现一些方法, 永远将 MyError 转化为 MyErrorResponse
impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            },
            MyError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal Server error".into()
            },
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            },
            MyError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
        }
    }
}

// 为 MyError 实现 ResponseError 这个trait, 这个 trait 就两个方法, 一个是 status_code, 另一个是 error_response
// 针对 MyError 这个自定义错误类型实现 error::ResponseError 这个trait 之后, 
// 只要发生了错误, actix就可以将错误信息转换为 http响应发送给客户端
// 但在实现  error::ResponseError 这个 trait 时候, 要求必须实现 Debug 和 Display 这两个 trait
// Debug 针对 MyError有一个默认的实现
impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            // 不管是服务器错误, 还是数据库错误, 都返回500
            MyError::DBError(_msg) | MyError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
            MyError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        // build方法传入status_code() 调用结果, 返回一个 HttpResponse, 然后继续调用 json 方法, 形成一段自定义的错误
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

// 手动实现 display
impl fmt::Display for  MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

// 转换错误信息, 使用 ? 即可
impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for MyError {
    fn from(err: SQLxError) -> Self {
        MyError::DBError(err.to_string())
    }
}
