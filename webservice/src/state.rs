// 应用程序的状态
// 由于使用了 actix 这个框架, 所以类似 AppState 这样的状态, 可以被注入到 请求他的 handler 中
// 所以 handler 可以通过参数来访问 AppState
use std::sync::Mutex;
// use super::models::Course;
use sqlx::postgres::PgPool;

pub struct AppState {
    // 健康检查, 不可变， 所有线程均持有
    pub health_check_response: String,
    // 可变的, 是一个数值, 这里使用 Mutex
    // ? Mutex 是标准库提供的一个保障线程通信的一个机制, 也就是在修改 visit_count 之前, 当前线程必须持有数据的控制权, 就是由 Mutex 完成的
    pub visit_count: Mutex<u32>,
    // pub courses: Mutex<Vec<Course>>,
    // 表示db是一个数据库连接池
    pub db: PgPool,
}

