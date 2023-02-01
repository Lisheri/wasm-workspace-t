// 已弃用
use super::errors::MyError;
use super::models::*;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows = sqlx::query!(
        r#"SELECT id, teacher_id, name, time FROM course WHERE teacher_id = $1"#,
        teacher_id
    )
    .fetch_all(pool) // fetch_all 用于查询多笔记录
    // 这个 ? 会在发生错误的时候将错误传递到 handler 中进行处理
    // 但是需要指明错误类型, 指明的方式就是返回值, 修改为 Result<T, E>
    .await?;

    let courses: Vec<Course> = rows
        .iter()
        .map(|r| Course {
            id: Some(r.id as usize),
            teacher_id: r.teacher_id as usize, // 断言为 usize
            name: r.name.clone().into(),
            time: Some(NaiveDateTime::from(r.time.unwrap())),
        })
        .collect();

    // 集合长度为0 , 其实就应该是 Not Found, 大于0, 就是正常返回
    match courses.len() {
        0 => Err(MyError::NotFound("Course not found for teacher".into())),
        _ => Ok(courses),
    }
}

pub async fn get_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
) -> Result<Course, MyError> {
    let row = sqlx::query!(
        r#"SELECT id, teacher_id, name, time FROM course WHERE teacher_id = $1 AND id = $2"#,
        teacher_id,
        id
    )
    .fetch_one(pool) // fetch_all 用于查询多笔记录
    .await;

    // 这里使用 if let 进行匹配
    if let Ok(row) = row {
        Ok(Course {
            id: Some(row.id as usize),
            teacher_id: row.teacher_id as usize,
            name: row.name.clone().into(),
            time: Some(NaiveDateTime::from(row.time.unwrap())),
        })
    } else {
        Err(MyError::NotFound("Course Id is not found".into()))
    }
}

/**
 * 新增course
 * @param pool 数据库连接池
 * @param new_course 新增的课程
 * @return course 新增的课程
 */
pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Result<Course, MyError> {
    // ? 通过 INSERT 插入到 course中, 插入 id, teacher_id, name, time自己生成, 然后通过 RETURNING 返回 id, teacher_id, name, time
    let row = sqlx::query!(
        r#"INSERT INTO course (id, teacher_id, name)
    VALUES ($1, $2, $3)
    RETURNING id, teacher_id, name, time"#,
        new_course.id.unwrap() as i32,
        new_course.teacher_id as i32,
        new_course.name
    )
    .fetch_one(pool)
    // 这里直接跟 ? 即可, 如果有错误会直接返回 Result<Error> 信息
    .await?;

    Ok(Course {
        id: Some(row.id as usize),
        teacher_id: row.teacher_id as usize,
        name: row.name.clone().into(),
        time: Some(NaiveDateTime::from(row.time.unwrap())),
    })
}
