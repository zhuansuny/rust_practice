use actix_web::{web,App,HttpResponse,HttpServer,Responder};
use std::io;

//配置route
pub fn general_routes(cfg: &mut web::ServiceConfig){
    cfg.route("/health",web::get().to(health_check_handler));

}
//配置 handle
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("vActix web Service is running")
}

//实例话 http server 并运行
#[actix_rt::main]
async fn  main() -> io::Result<()> {
    // 构建 app, 配置route
    let app = move || App::new().configure(general_routes);

    // 运行server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
    
}