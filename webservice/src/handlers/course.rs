use crate::db_access::course::*;
use crate::state::AppState;
use crate::errors::MyError;
use actix_web::{web, HttpResponse};

use crate::models::course::{CreateCourse, UpdateCourse};

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("Received new course.");
    /* let course_count = app_state
        .courses
        .lock() // 获取权限
        .unwrap()
        .clone()
        .into_iter() // 将courses 变成一个迭代器
        .filter(|course| course.teacher_id == new_course.teacher_id)
        .collect::<Vec<Course>>()
        .len();

    let new_course = Course {
        teacher_id: new_course.teacher_id,
        // ? 当前老师的课程自增, 上面筛选了新增的是哪个老师的课程
        id: Some(course_count + 1), // id 自增即可, 保持唯一, 所有线程共同持有同一个 courses
        name: new_course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    // 将新的 course 传进去
    app_state.courses.lock().unwrap().push(new_course);*/
    // 调用 post_new_course_db 添加到数据库并返回添加的课程
    // ? CreateCourse 并没有实现 from, 而是实现的 try_from, 所以这里需要使用 try_into
    // ? 后面跟一个 ? 标识转换可能会出错, 简单处理一下
    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

// ? 无需转换, 已弃用
// 主要讲 usize 转换为 i32
// fn translate_usize_to_i32(param: usize) -> i32 {
//     return i32::try_from(param).unwrap();
// }

/**
 * 获取老师的所有课程
 * params web::Path<(usize)> 是个元组, 元组类型为 usize
 * ? 这里一样, params, 需要再unsize后加一个逗号, 否则不会作为元组编译, 因为元组内部只有一个值, 不建议作为容器类型
 */
pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    // params: web::Path<(usize,)>,
    params: web::Path<i32>,
    // params 参数可以修改为如下所示
    // web::Path(teacher_id): web::Path<i32> 
) -> Result<HttpResponse, MyError> {
    /* // 获取元组的第一个元素, 也就是teacher_id
    // let teacher_id: usize = params.0;
    let (teacher_id) = params.into_inner();
    // 过滤这个老师教的课程
    let filtered_course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == teacher_id)
        .collect::<Vec<Course>>();
    // 表示存在课程
    if filtered_course.len() > 0 {
        // 找到这个老师教的课程就返回
        HttpResponse::Ok().json(filtered_course)
    } else {
        // 没有找到就返回错误信息
        HttpResponse::Ok().json("No courses found for teacher".to_string())
    } */
    // 依然是先提取 teacher_id, 是一个元组, 这里需要转换为i32
    // ? 先通过 params.into_inner() 将元组转换为公有键的形式, 在读取第一个值, 也就是usize的 teacher_id
    // ? 然后通过 i32::from转换为 i32类型的 Result<T, E>, 经过 unwrap 快速处理一下返回
    // ? 上面直接使用 web::Path(teacher_id), 将 teacher_id从元组中抽取出来了
    // let teacher_id = translate_usize_to_i32(params.into_inner().0);
    // ? 无需转换
    let teacher_id = params.into_inner();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        // 这个表示如果成功, 就将 courses 返回出去
        // 如果失败就会发生错误, 得到的错误类型就是 MyError
        // 由于MyError实现了 ResponseError 这个 trait, 所以 Actix会把 MyError 自动转换为错误对应的响应信息转发给用户
        .map(|courses| HttpResponse::Ok().json(courses))
}

// 获取老师的某一个课程
pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    // params: web::Path<(usize, usize)>,
    params: web::Path<(i32, i32)>
) -> Result<HttpResponse, MyError> {
    /* let (teacher_id, course_id) = params.into_inner();
    // 查找这个老师的详细课程
    let selected_course = app_state.courses.lock().unwrap().clone().into_iter()
        // ? 这里需要判断 Option<usize> 全等, 但其实从路径传入的 course_id 是一个usize, 而不是 Option<usize>, 所以需要转换
        // ? 这里直接用find即可
        .find(|course| course.teacher_id == teacher_id && course.id == Some(course_id))
        .ok_or("Course not found"); // 转换为一个 Result<T, E>类型, 如果接收的是Some, 就返回 Ok(course) , 否则就是 Err

    // 判断返回的结果是否是 Ok的
    if let Ok(course) = selected_course {
        HttpResponse::Ok().json(course)
    } else {
        HttpResponse::Ok().json("Course not found!".to_string())
    } */
    // let params_tuple = params.into_inner();
    // let (teacher_id, course_id) = (
    //     translate_usize_to_i32(params_tuple.0),
    //     translate_usize_to_i32(params_tuple.1),
    // );
    let (teacher_id, course_id) = params.into_inner();
    get_course_details_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

// 删除课程
pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
    .await
    .map(|res| HttpResponse::Ok().json(res))
}

// 更新
pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    // params: web::Path<(usize, usize)>,
    params: web::Path<(i32, i32)>
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    // 提取 update_course时, 需要调用一次 into, 因为 updateCourse 实现的是 from trait
    update_course_details_db(&app_state.db, teacher_id, course_id, update_course.into())
    .await
    .map(|res| HttpResponse::Ok().json(res))
}

// 测试
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    // 异步测试, 需要使用 actix_rt 这个异步运行时
    #[ignore] // 忽略当前测试, 通过一次后, 数据已经插入
    #[actix_rt::test]
    // 这个用例暂时只能跑一次, 需要重跑的话就要换个id, 或者删除添加的数据
    async fn post_course_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let course = web::Json(CreateCourse {
            teacher_id: 1,
            name: "Test Course".to_string().into(), // &str 不能直接调用into进行转换, 除非Trait实现了 AsRef, 允许 &str与String共享From
            description: Some("This is a course".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("English".into()),
            level: Some("Beginner".into()),
        });
        let res = post_new_course(course, app_state).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        // ? 这里不加一个逗号, 不会被当做元组编译
        let teacher_id: web::Path<i32> = web::Path::from(1);
        // 简单处理, 直接 unwrap() 取出结果
        let res = get_courses_for_teacher(app_state, teacher_id)
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() { 
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let res = get_course_detail(app_state, params).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        // course_id不存在
        let params: web::Path<(i32, i32)> = web::Path::from((1, 100));
        let res = get_course_detail(app_state, params).await;
        match res {
            Ok(_) => println!("Something wrong..."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND)
        }
    }

    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let update_course: UpdateCourse = UpdateCourse {
            name: Some("Courese name changed".into()),
            description: Some("This is another test course".into()),
            format: None,
            structure: None,
            duration: None,
            price: Some(32),
            language: Some("Chinese".into()),
            level: Some("Intermediate".into()),
        };
        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let json_update_course = web::Json(update_course);
        let res = update_course_details(app_state, json_update_course, params)
        .await
        .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[ignore]
    #[actix_rt::test]
    async fn delete_course_success() {
        // 删除成功
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 3));
        let res = delete_course(app_state, params)
        .await
        .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_course_failure() {
        // 删除成功
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not defined");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 10000));
        let res = delete_course(app_state, params).await;
        match res {
            Ok(_) => println!("Something wrong..."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND)
        }
    }
}
