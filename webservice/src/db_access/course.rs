use crate::errors::MyError;
use crate::models::course::{Course, UpdateCourse, CreateCourse};
// use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    // let rows = sqlx::query!(
    //     r#"SELECT id, teacher_id, name, time FROM course WHERE teacher_id = $1"#,
    //     teacher_id
    // )
    // .fetch_all(pool) // fetch_all 用于查询多笔记录
    // 这个 ? 会在发生错误的时候将错误传递到 handler 中进行处理
    // 但是需要指明错误类型, 指明的方式就是返回值, 修改为 Result<T, E>
    // .await?;
    // ? 这里直接使用 query_as, 将结果转换为Vec<Course> 类型, 因为在声明Course时, 添加了 FromRow, 用于数据库读取后直接映射
    // ? query_as 第一个参数是我们需要的类型
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"
        SELECT * FROM course
        WHERE teacher_id = $1"#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;
    // ? 无需在遍历创建courses了, 上面可以直接从数据库中取出 Course 类型的 Vector
    // let courses: Vec<Course> = rows
    //     .iter()
    //     .map(|r| Course {
    //         id: Some(r.id as usize),
    //         teacher_id: r.teacher_id as usize, // 断言为 usize
    //         name: r.name.clone().into(),
    //         time: Some(NaiveDateTime::from(r.time.unwrap())),
    //     })
    //     .collect();

    // 集合长度为0 , 其实就应该是 Not Found, 大于0, 就是正常返回
    match rows.len() {
        0 => Err(MyError::NotFound("Course not found for teacher".into())),
        _ => Ok(rows),
    }
}

pub async fn get_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
) -> Result<Course, MyError> {
    let row: Option<Course> = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE teacher_id = $1 AND id = $2"#,
        teacher_id,
        id
    )
    // 这里使用 fetch_optional, 返回的是一个 options 类型, 表示可能查询到, 可能查询不到
    .fetch_optional(pool)
    .await?;

    // 这里使用 if let 进行判断, 因为前面返回的是一个 Option 枚举
    if let Some(course) = row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course Id or Teacher Id is not found".into()))
    }
}

/**
 * 新增course
 * @param pool 数据库连接池
 * @param new_course 新增的课程
 * @return course 新增的课程
 */
pub async fn post_new_course_db(pool: &PgPool, new_course: CreateCourse) -> Result<Course, MyError> {
    // ? 通过 INSERT 插入到 course中, 插入 id, teacher_id, name, time自己生成, 然后通过 RETURNING 返回 id, teacher_id, name, time
    let row = sqlx::query_as!(
        Course,
        r#"INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level"#,
        new_course.teacher_id, new_course.name, new_course.description, new_course.format,
        new_course.structure, new_course.duration, new_course.price, new_course.language, new_course.level
    )
    .fetch_one(pool)
    // 这里直接跟 ? 即可, 如果有错误会直接返回 Result<Error> 信息
    .await?;

    Ok(row)
}

/**
 * 删除 course
 */

pub async fn delete_course_db(pool: &PgPool, teacher_id: i32, id: i32) -> Result<String, MyError> {
    let course_row = sqlx::query!( 
    "DELETE FROM course where id = $1 and teacher_id = $2",
    id, teacher_id
    )
    // 执行删除
    .execute(pool)
    .await?;

    Ok(format!("Delete {:?} record", course_row))
}

pub async fn update_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse
) -> Result<Course, MyError> {
    let current_course_row = sqlx::query_as!(Course, r#"
        SELECT * FROM course where id = $1 and teacher_id = $2
    "#,
    id, teacher_id)
    .fetch_one(pool)
    .await
    // 如果没有查到就返回一个错误 not found
    .map_err(|_err| MyError::NotFound("Course id not found".into()))?;

    // 如果 update_course.name 没有值, 那么说明name没有进行更新, 此时直接获取 current_course_row.name 即可
    let name: String = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name
    };

    // ? 其余同上
    let description: String = if let Some(description) = update_course.description {
        description
    } else {
        // ? 这里要解除 Option, 转换为 String
        current_course_row.description.unwrap_or_default()
    };

    let format: String = if let Some(format) = update_course.format {
        format
    } else {
        current_course_row.format.unwrap_or_default()
    };

    let structure: String = if let Some(structure) = update_course.structure {
        structure
    } else {
        current_course_row.structure.unwrap_or_default()
    };

    let duration: String = if let Some(duration) = update_course.duration {
        duration
    } else {
        current_course_row.duration.unwrap_or_default()
    };

    let price: i32 = if let Some(price) = update_course.price {
        price
    } else {
        current_course_row.price.unwrap_or_default()
    };

    let language: String = if let Some(language) = update_course.language {
        language
    } else {
        current_course_row.language.unwrap_or_default()
    };

    let level: String = if let Some(level) = update_course.level {
        level
    } else {
        current_course_row.level.unwrap_or_default()
    };
    
    let course_row = sqlx::query_as!(
        Course,
        r#"
            UPDATE course SET name = $1, description = $2, format = $3, 
            structure = $4, duration = $5, price = $6, language = $7, level = $8
            where id = $9 and teacher_id = $10
            RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level
        "#,
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        id,
        teacher_id
    )
    .fetch_one(pool)
    .await;

    if let Ok(course) = course_row{
        Ok(course)
    } else {
        Err(MyError::NotFound("Course is not found".into()))
    }
}
