use actix_web::web;
use serde::{Deserialize, Serialize}; // 反序列化 和 序列化

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Teacher {
    pub id: i32,
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

// 新增和编辑均不需要序列化, 只需要反序列化
#[derive(Deserialize, Debug, Clone)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

// 更新时可以不传, 说明当前字段不需要更新, 所以属性可以是None, 使用 Option 枚举
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTeacher {
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

impl From<web::Json<CreateTeacher>> for CreateTeacher {
    fn from(new_teacher: web::Json<CreateTeacher>) -> Self {
        CreateTeacher {
            name: new_teacher.name.clone(),
            picture_url: new_teacher.picture_url.clone(),
            profile: new_teacher.profile.clone()
        }
    }
}

impl From<web::Json<UpdateTeacher>> for UpdateTeacher {
    fn from(update_teacher: web::Json<UpdateTeacher>) -> Self {
        UpdateTeacher {
            name: update_teacher.name.clone(),
            picture_url: update_teacher.picture_url.clone(),
            profile: update_teacher.profile.clone()
        }
    }
}
