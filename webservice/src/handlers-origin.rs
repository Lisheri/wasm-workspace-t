// 原始handler 已弃用
use super::db_access::*;
use crate::errors::MyError;
use crate::models::course::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};

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

pub async fn new_course(
    new_course: web::Json<Course>,
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
    post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

fn translate_usize_to_i32(param: usize) -> i32 {
    return i32::try_from(param).unwrap();
}

/**
 * 获取老师的所有课程
 * params web::Path<(usize)> 是个元组, 元组类型为 usize
 * ? 这里一样, params, 需要再unsize后加一个逗号, 否则不会作为元组编译, 因为元组内部只有一个值, 不建议作为容器类型
 */
pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>,
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
    let teacher_id = translate_usize_to_i32(params.into_inner().0);
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
    params: web::Path<(usize, usize)>,
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
    let params_tuple = params.into_inner();
    let (teacher_id, course_id) = (
        translate_usize_to_i32(params_tuple.0),
        translate_usize_to_i32(params_tuple.1),
    );
    get_course_details_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

// 测试
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
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
        let course = web::Json(Course {
            teacher_id: 1,
            name: "Test Course".to_string().into(), // &str 不能直接调用into进行转换, 除非Trait实现了 AsRef, 允许 &str与String共享From
            id: Some(6), // 这里不能继续使用 None 了, 数据库设置时不允许使用null
            time: None,
        });
        let res = new_course(course, app_state).await.unwrap();
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
        let teacher_id: web::Path<(usize,)> = web::Path::from((1,));
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
        let params: web::Path<(usize, usize)> = web::Path::from((1, 1));
        let res = get_course_detail(app_state, params).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
}
