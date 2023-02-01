// use super::super::log;
use crate::errors::MyError;
use wasm_bindgen::prelude::*;
use chrono::NaiveDateTime;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
// use crate::JsValue;
use js_sys::Promise;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
// 这个 Course, 只需要在Rust代码中访问, 所以不需要 wasm_bindgen 这个 attribute
pub struct Course {
    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: NaiveDateTime,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

pub async fn get_courses_by_teacher(teacher_id: i32) -> Result<Vec<Course>, MyError> {
    // 创建一个 request
    let mut opts = RequestInit::new();
    opts.method("GET");
    // 需要跨域
    opts.mode(RequestMode::Cors);

    let url = format!("http://localhost:3000/courses/{}", teacher_id);

    // 发起请求
    let request = Request::new_with_str_and_init(&url, &opts)?;
    // 设置header
    request.headers().set("Accept", "application/json")?;
    // 获取浏览器的 window 对象
    let window = web_sys::window().ok_or("no window exists".to_string())?;
    // 获取响应数据
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // 查看返回值是否为Response的实例
    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    // 序列化
    let courses: Vec<Course> = json.into_serde().unwrap();

    Ok(courses)
}

pub async fn delete_course(teacher_id: i32, course_id: i32) -> () {
    // 请求设置
    let mut opts = RequestInit::new();
    opts.method("DELETE");
    opts.mode(RequestMode::Cors);

    let url = format!("http://localhost:3000/courses/{}/{}", teacher_id, course_id);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request.headers().set("Accept", "application/json").unwrap();

    let window = web_sys::window().ok_or("no window exists".to_string()).unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

    // 这里需要对结果进行转换, 但不需要使用, 可以在转换过程中根据是否出错来确定函数是否有问题
    let _courses: Course = json.into_serde().unwrap();
}

#[wasm_bindgen]
// JsValue 表示错误
pub async fn add_course(name: String, description: String) -> Result<Promise, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    let str_json = format!(r#"
        {{
            "teacher_id": 1,
            "name": "{}",
            "description": "{}"
        }}
    "#, name, description);
    opts.body(Some(&JsValue::from_str(str_json.as_str())));
    let url = format!("http://localhost:3000/courses/");
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Content-Type", "application/json")?;
    request.headers().set("Accept", "application/json")?;
    let window = web_sys::window().ok_or("no window exists".to_string())?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    // 这个 resp.json 返回的就是一个Promise, 对应的就是 js 中的 Promise
    Ok(resp.json()?)

}
