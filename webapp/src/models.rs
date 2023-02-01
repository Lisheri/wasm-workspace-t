use serde::{Deserialize, Serialize};

// 教师注册
#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherRegisterForm {
    pub name: String,
    pub image_url: String,
    pub profile: String,
}

// 查询老师返回的结果(包含id的)
#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherResponse {
    pub id: i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}
