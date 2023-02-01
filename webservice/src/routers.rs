use super::handlers::course::*;
use super::handlers::teacher::*;
use crate::handlers::general::health_check_handler;
use actix_web::web;

// 健康检查
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// 注册课程路由
pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // 这类用 service配合 web::scope 限制了一套资源的根路径, 也就是 /courses
        // 在其下方可以继续添加资源
        .service(
            web::scope("/courses")
                .route("/", web::post().to(post_new_course))
                // 添加路由, 动态路由, user_id也就是teacher_id
                .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
                .route("/{teacher_id}/{course_id}", web::get().to(get_course_detail))
                .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
                .route("/{teacher_id}/{course_id}", web::put().to(update_course_details))
        );
}

pub fn teacher_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/teachers")
            .route("", web::post().to(post_new_teacher))
            .route("", web::get().to(get_all_teachers))
            .route("/{teacher_id}", web::get().to(get_teacher_details))
            .route("/{teacher_id}", web::put().to(update_teacher_details))
            .route("/{teacher_id}", web::delete().to(delete_teacher))
        );
}
