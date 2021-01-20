use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod errors;
use errors::ApiError;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> Result<HttpResponse, ApiError> {
    Err(ApiError::ValidationError(String::from("No way!")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let scope = web::scope("/api")
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello));

        App::new().service(scope)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
