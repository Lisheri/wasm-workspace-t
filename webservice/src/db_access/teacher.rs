use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};
use sqlx::postgres::PgPool;

pub async fn get_all_teachers_db(pool: &PgPool) -> Result<Vec<Teacher>, MyError> {
    let rows = sqlx::query!(r#"SELECT id, name, picture_url, profile FROM teacher"#)
        .fetch_all(pool)
        .await?;

    let teachers: Vec<Teacher> = rows
        .iter()
        .map(|row| Teacher {
            id: row.id,
            name: row.name.clone(),
            picture_url: row.picture_url.clone(),
            profile: row.profile.clone(),
        })
        .collect();

    match teachers.len() {
        0 => Err(MyError::NotFound("No teachers found".into())),
        _ => Ok(teachers),
    }
}

pub async fn get_teacher_details_db(pool: &PgPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let row: Option<Teacher> = sqlx::query_as!(
        Teacher,
        r#"
        SELECT id, name, picture_url, profile FROM teacher
        WHERE id = $1"#,
        teacher_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(teacher) = row {
        Ok(teacher)
    } else {
        Err(MyError::NotFound("Teacher is not found".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_teacher: CreateTeacher) -> Result<Teacher, MyError> {
    let row: Teacher = sqlx::query_as!(Teacher, r#"
        INSERT INTO teacher (name, picture_url, profile) VALUES ($1, $2, $3)
        RETURNING id, name, picture_url, profile
    "#, new_teacher.name, new_teacher.picture_url, new_teacher.profile)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn update_teacher_details_db(pool: &PgPool, teacher_id: i32, update_teacher: UpdateTeacher) -> Result<Teacher, MyError> {
    let current_teacher = sqlx::query_as!(Teacher, r#"
        SELECT * FROM teacher WHERE id = $1
    "#, teacher_id)
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Course id not found".into()))?;

    let name: String = if let Some(name) = update_teacher.name {
        name
    } else {
        current_teacher.name.unwrap_or_default()
    };

    let picture_url: String = if let Some(picture_url) = update_teacher.picture_url {
        picture_url
    } else {
        current_teacher.picture_url.unwrap_or_default()
    };

    let profile: String = if let Some(profile) = update_teacher.profile {
        profile
    } else {
        current_teacher.profile.unwrap_or_default()
    };

    let current_row = sqlx::query_as!(Teacher, r#"
        UPDATE teacher SET name = $1, picture_url = $2, profile = $3
        WHERE id = $4
        RETURNING id, name, picture_url, profile
    "#, name, picture_url, profile, teacher_id)
    .fetch_one(pool)
    .await;

    if let Ok(teacher) = current_row {
        Ok(teacher)
    } else {
        Err(MyError::NotFound("Teacher is not found".into()))
    }
}

pub async fn delete_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<String, MyError> {
    let teacher_row = sqlx::query!(r#"
        DELETE FROM teacher WHERE id = $1
    "#, teacher_id)
    .execute(pool)
    .await
    .map(|_err| MyError::DBError("Unable to delete teacher".into()));
    Ok(format!("Delete {:?} record", teacher_row))
}
