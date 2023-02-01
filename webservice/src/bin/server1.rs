// 这里面包含 server1 这个二进制的 main 函数
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// 配置 route
pub fn general_route(cfg: &mut web::ServiceConfig) {
    // ? 参数cfg 是一个 web服务配置选项
    // ? route 方法第一个参数是 url
    // ? web::get() 表示用的方法是 http get 方法
    // ? health_check_handler 就是对应的 handler, 实现在下面
    cfg.route("/health", web::get().to(health_check_handler));
}

pub async fn health_check_handler() -> impl Responder {
    // 返回的就是 Ok 这个 response, 也就是200, 然后返回一个 json, 表示 web 服务正在运行
    // 其实就是判断这个web 服务是否正常
    // ? 要求返回结果实现 Responder 这个 trait
    HttpResponse::Ok().json("Actix Web Service is running!")
}


// 实例化 HTTP server 并运行
// 这个注解就运用到了 actix_rt 这个运行时下的 main 函数
#[actix_rt::main]
// 是一个异步函数
async fn main() -> io::Result<()> {
    // 构建 app, 配置 route
    // ? 创建 actix 应用, 同时配置路由
    let app = move || App::new().configure(general_route);

    // 运行 HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
    // 运行方式有两种
    // 第一种是在 workspace 下, 使用 cargo run -p webservice(项目名) --bin server1(bin名)
    // 第二种是进入项目中, cd webservice && cargo run --bin server1
}

// Actix HTTP Server 其实就是实现了一个 HTTP 协议, 用于应对所有的请求
// 默认情况下开启多个线程处理进来的请求
// 支持两类并发:
// 1. 异步I/O: 给定的 OS 原生线程在等待 I/O 时执行其他任务(例如侦听网络连接)
// 2. 多线程并行: 默认情况下启动 OS 原生线程的数量与系统逻辑CPU数量相同
