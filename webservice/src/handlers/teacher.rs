use crate::db_access::teacher::*;
use crate::errors::MyError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

use crate::models::teacher::{CreateTeacher, UpdateTeacher};

// * 查询全部教师
pub async fn get_all_teachers(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    get_all_teachers_db(&app_state.db)
        .await
        .map(|teachers| HttpResponse::Ok().json(teachers))
}

// * 获取老师详细信息
pub async fn get_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_teacher_details_db(&app_state.db, teacher_id)
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

// * 新增老师
pub async fn post_new_teacher(
    new_teacher: web::Json<CreateTeacher>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_course_db(&app_state.db, CreateTeacher::from(new_teacher))
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn update_teacher_details(
    app_state: web::Data<AppState>,
    update_teacher: web::Json<UpdateTeacher>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    update_teacher_details_db(&app_state.db, teacher_id, update_teacher.into())
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    delete_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

// * 测试
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_teachers_success_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let res = get_all_teachers(app_state).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_teacher_details_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<i32> = web::Path::from(1);
        let res = get_teacher_details(app_state, params).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_teacher_success_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        
        let new_teacher = web::Json(CreateTeacher {
            name: "张三".into(),
            picture_url: "https://commonresource-1252524126.cdn.xiaoeknow.com/image/l2y5zx530z40.png".into(),
            profile: "高级教师".into()
        });

        let res = post_new_teacher(new_teacher, app_state).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[ignore]
    #[actix_rt::test]
    async fn delete_teacher_success_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<i32> = web::Path::from(100);
        let res = delete_teacher(app_state, params).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn update_teacher_success_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let update_teacher_json = web::Json(UpdateTeacher {
            name: Some("新李四".into()),
            picture_url: Some("https://commonresource-1252524126.cdn.xiaoeknow.com/image/l2y5zx530z40.png".into()),
            profile: None
        });
        let params: web::Path<i32> = web::Path::from(200);
        let res = update_teacher_details(app_state, update_teacher_json, params).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
}
