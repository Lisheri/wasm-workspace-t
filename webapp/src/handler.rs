use crate::errors::MyError;
use crate::models::{TeacherRegisterForm, TeacherResponse};
use actix_web::{web, Error, HttpResponse, Result};
use serde_json::json;
use tera::Context;
use awc::Client;

pub struct Config {
    baseUrl: String,
}

// pub let config = Config {
//     baseUrl: "http://localhost:3000"
// }

fn get_context() -> Context {
    tera::Context::new()
}

fn get_default_client() -> Client {
    awc::Client::default()
}

pub async fn get_all_teachers(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    // 创建一个 http 客户端, 用它可以访问到 webService 下的东西
    let awc_client = get_default_client();
    let res = awc_client
        .get("http://localhost:3000/teachers")
        .send()
        .await
        .unwrap()
        // 将获取的json数据转换为 Vec<TeacherResponse>
        .json::<Vec<TeacherResponse>>()
        .await
        .unwrap();

    // 创建一个上下文, 用于向 html 模板内添加数据
    let mut ctx = get_context();
    ctx.insert("error", "");
    ctx.insert("teachers", &res);

    // 渲染模板, 这里就是找 static/teachers.html
    let s = tmpl.render("teachers.html", &ctx)
        .map_err(|_| MyError::TeraError("Template Error".to_string()));
    // 返回响应
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}

// 展示注册表单页面
pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = get_context();
    ctx.insert("error", "");
    ctx.insert("current_name", "");
    ctx.insert("current_image_url", "");
    ctx.insert("current_profile", "");
    let s = tmpl.render("register.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()));
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}

// 注册教师
pub async fn handle_register(tmpl: web::Data<tera::Tera>, params: web::Form<TeacherRegisterForm>) -> Result<HttpResponse, Error> {
    let mut ctx = get_context();
    let s;
    if params.name == "Dave" {
        ctx.insert("error", "Dave is already exists!");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_profile", &params.profile);
        ctx.insert("current_image_url", &params.image_url);
        s = tmpl.render("register.html", &ctx)
            .map_err(|_| MyError::TeraError("Template Error".to_string())).unwrap();
    } else {
        let new_teacher = json!({
            // 这里面的字段要和后端服务中新增老师的字段对应上
            "name": &params.name,
            "picture_url": &params.image_url,
            "profile": &params.profile
        });
        
        let awc_client = get_default_client();
        let res = awc_client
            .post("http://localhost:3000/teachers")
            .send_json(&new_teacher)
            .await
            .unwrap()
            .body()
            .await
            .unwrap();
        // 将上面获取的结果转换为一个字符串切片, 再从字符串切片转换为一个 TeacherResponse 格式的 json
        let teacher_response: TeacherResponse = serde_json::from_str(&std::str::from_utf8(&res).unwrap()).unwrap();
        s = format!("Congratulation! Your id is: {}.", teacher_response.id);
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
