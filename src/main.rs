mod route;
mod config;
mod model;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};

#[get("/")]
async fn hello() -> impl Responder {
    let logins = route::route::login();
    print!("{}", logins.await);
    HttpResponse::Ok().body("Test")
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
    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    }).workers(1)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}