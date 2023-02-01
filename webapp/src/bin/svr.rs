#[path = "../mod.rs"]
mod wa; // wa: webApplication
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routers::app_config;
use std::env;
use wa::{errors, handler, models, routers};

use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 读取env
    dotenv().ok();
    let host_port = env::var("HOST_PORT").expect("HOST_PORT address is occupied");
    println!("Listening on {}", &host_port);
    HttpServer::new(move || {
        // CARGO_MAINFEST_DIR 这个环境变量对应的值就对应到建立 webapp对应的绝对路径, 然后寻找下面的 static/**/* 内容
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        // 创建服务并注册路由
        App::new()
            .app_data(web::Data::new(tera))
            .configure(app_config)
    })
    .bind(&host_port)? // 绑定到端口
    .run() // 运行
    .await
}
