use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize}; // 反序列化 和 序列化
use crate::errors::MyError;
use std::convert::TryFrom;

// 移动进来以后, 引用路径就变成了 use crate::models::course::Course
// FromRow 用于在添加数据库后, 从数据库读取数据时, 自动将数据库表的数据映射为 Crouse 这个 struct
// ? 反序列化也不需要了, Course 只需要存储数据库读取的结果, 并不用于新增或者修改, 因此可以去掉
// ? Course 只需要从数据库读取出来之后进行序列化即可
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub teacher_id: i32,
    pub id: i32, // 由于新增Course时数据库会生成id, 所以不需要使用Option枚举, i32 类型即可
    pub name: String,
    pub time: Option<NaiveDateTime>, // 允许是None的日期时间类型

    // 新增字段全是 Option 的, 所以他们是可空的
    pub description: Option<String>, // 描述
    pub format: Option<String>,      // 格式 安排进度或者自行安排进度, 直播课, 线下课等
    pub structure: Option<String>,   // 课程的结构
    pub duration: Option<String>,    // 课程持续时间, 单位可能不同, 所以使用字符串
    pub price: Option<i32>,          // 价格
    pub language: Option<String>,    // 语言
    pub level: Option<String>,       // 等级, 初级 中级 高级 等
}

// ? 新增专用 struct
// ? id 和 time 都是数据库生成的, 所以不需要我们单独实现
#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: String,
    pub description: Option<String>, // 描述
    pub format: Option<String>,      // 格式 安排进度或者自行安排进度, 直播课, 线下课等
    pub structure: Option<String>,   // 课程的结构
    pub duration: Option<String>,    // 课程持续时间, 单位可能不同, 所以使用字符串
    pub price: Option<i32>,          // 价格
    pub language: Option<String>,    // 语言
    pub level: Option<String>,       // 等级, 初级 中级 高级 等
}

// 修改课程, 老师不能修改, 所以不需要 teacher_id
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>, // 因为更新课程时, 下面的所有属性都可能不进行更新, 所以他是一个 Option, 不更新的时候就是空值
    pub description: Option<String>, // 描述
    pub format: Option<String>,      // 格式 安排进度或者自行安排进度, 直播课, 线下课等
    pub structure: Option<String>,   // 课程的结构
    pub duration: Option<String>,    // 课程持续时间, 单位可能不同, 所以使用字符串
    pub price: Option<i32>,          // 价格
    pub language: Option<String>,    // 语言
    pub level: Option<String>,       // 等级, 初级 中级 高级 等
}


// 这里需要的是 From<web::Json>到 CreateCourse, 而Course不需要实现 From trait 了
// impl From<web::Json<CreateCourse>> for CreateCourse {
//     fn from(course: web::Json<CreateCourse>) -> Self {
//         // 因为进来的是 json 格式的数据, 这里需要通过 from 将 json 格式的数据转换为 Course
//         CreateCourse {
            // teacher_id: course.teacher_id,
            // name: course.name.clone(),
            // description: course.description.clone(),
            // format: course.format.clone(),
            // structure: course.structure.clone(),
            // duration: course.duration.clone(),
            // price: course.price,
            // language: course.language.clone(),
            // level: course.level.clone(),
//         }
//     }
// }
// 这里改 From 为 TryFrom, 这两个会冲突, 需要去掉 From
impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = MyError;
    // 转换稍微不是那么直接, 会自动处理错误信息, 防止 panic
    // 这里如果失败的话, 会直接返回 Self::Error, 也就是 MyError
    fn try_from(course: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(CreateCourse {
            teacher_id: course.teacher_id,
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        })
    }
}

// 修改课程用 From trait即可, 一般不会失败
impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(course: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        }
    }
}

/* impl From<web::Json<Course>> for Course {
    // ? web::Json 和 web::Data 有点像, 他们其实都是数据提取器, 可以将 json 数据提取为传入的类型
    fn from(course: web::Json<Course>) -> Self {
        // 因为进来的是 json 格式的数据, 这里需要通过 from 将 json 格式的数据转换为 Course
        Course {
            teacher_id: course.teacher_id,
            id: course.id,
            name: course.name.clone(), // 需要有所有权, 在一个 Option<String> 类型不 clone, 就会发生移动, 所有权直接转移了, 可能会导致变量被回收
            time: course.time
        }
    }
} */
