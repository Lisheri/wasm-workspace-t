use crate::state::AppState;
use actix_web::{web, HttpResponse};

// 健康检查
pub async fn health_check_handler(
    // 注入的 AppState, 只要 AppState 在 actix 中注册过, 就可以在 handler 进行注入, 其实就是通过 web::Data<AppState>
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    // 访问可变字段, 因为他用 Mutex 包裹, 所以必须先lock, 上锁, 防止其他线程在对这个值进行操作, 然后使用 unwrap 处理一下lock可能产生的错误, 也就是有线程正在操作当前值
    let mut visit_count = app_state.visit_count.lock().unwrap();
    // 形成一个响应
    let response = format!("{} {} times", health_check_response, visit_count);
    // 更新 visit_count
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
    // 走完这个 handler, 上面的锁就自动释放了
}