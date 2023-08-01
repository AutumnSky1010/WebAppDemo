use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn top() -> impl Responder {
    HttpResponse::Ok().body("hello world!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(top)
    });
    let result = server.bind(("127.0.0.1", 8080))?.run().await;
    return result;
}
