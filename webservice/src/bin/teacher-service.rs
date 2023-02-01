use actix_web::{web, App, HttpServer, http};
use dotenv::dotenv;
use std::env;
use std::io;
use std::sync::Mutex;
use actix_cors::Cors;

// 定义模块
#[path = "../db_access/mod.rs"]
mod db_access;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../errors.rs"]
mod errors;

use routers::*;
use sqlx::{postgres::PgPoolOptions, Executor};
use state::AppState;

use crate::errors::MyError;

// 异步 main
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 读取环境变量
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
    // 创建数据库连接池
    let db_pool = PgPoolOptions::new()
        .after_connect(|conn, _x| {
            Box::pin(async move {
                conn.execute("SET TIME ZONE 'Asia/Shanghai';").await?;
                Ok(())
            })
        })
        .connect(&database_url)
        .await
        .expect("Could not create database pool");
    // 创建共享state
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    // app是一个闭包, 就是创建一个 web 应用
    let app = move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080/")
            .allowed_origin_fn(|origin, _req_head| {
                // 允许所有以 localhost 开头的域
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "DELETE"]) // 允许的请求方法
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE) // 允许的请求头
            .max_age(3600); // 3600s未响应就截断

        App::new()
            // 注入 注册共享state, 此时就可以向 handler 中注入数据了
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                // 注册拦截不合法请求, 如果检测到前端传递不合法输入, 就会进入
                MyError::InvalidInput("Please provide valid json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .wrap(cors)
            .configure(teacher_routes) // 注册老师路由
    };
    println!("监听到了端口 localhost:3000");
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
