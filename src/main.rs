use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, guard, web};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!\n", name)
}

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet).guard(guard::Get()))
            // .route("/{name}", web::get().to(greet))
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
