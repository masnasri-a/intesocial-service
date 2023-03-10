mod route;
mod config;
mod model;
mod util;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use serde::Deserialize;


#[derive(Deserialize)]
struct LoginInfo {
    username: String,
    password:String,
}

#[derive(Deserialize)]
struct RegisterInfo{
    username:String,
    full_name:String,
    email:String,
    password:String,
}


#[post("/login")]
async fn login(info:web::Json<LoginInfo>) -> impl Responder {
    let login_info: LoginInfo = info.into_inner();
    let logins = route::route::login(login_info.username, login_info.password);
    HttpResponse::Ok().body(logins.await)
}


#[post("/register")]
async fn register(info:web::Json<RegisterInfo>) -> impl Responder {
    let register_info: RegisterInfo = info.into_inner();
    let register_result = route::route::register(&register_info.username,&register_info.full_name,&register_info.email, &register_info.password);
    HttpResponse::Ok().body(register_result.await)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
            .service(login)
            .service(register)
            .route("/hey", web::get().to(manual_hello))
    }).workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}