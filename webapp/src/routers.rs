use actix_files as fs;
use actix_web::web;
use crate::handler::{get_all_teachers, show_register_form, handle_register};

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            // 所有的html, css, js等都是在 static 目录下的静态文件
            // ? 前一个 /static 是url路径, 访问这个url, 就会找 ./static下面的所有文件
            .service(fs::Files::new("/static", "./static").show_files_listing())
            // 访问 / 就会走到 get_all_teachers
            .service(web::resource("/").route(web::get().to(get_all_teachers)))
            // 显示注册的页面
            .service(web::resource("/register").route(web::get().to(show_register_form)))
            // 注册老师
            .service(web::resource("/register-post").route(web::post().to(handle_register)))
    );
}
