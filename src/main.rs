use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs;

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

async fn manual_hello(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let jwt_secret = &data.jwt_secret;
    Err(ApiError::ValidationError(jwt_secret.to_string()))
}

struct AppState {
    jwt_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let jwt_secret = fs::read_to_string("jwt.secret")
        .expect("JWTSecret is neccesary to start this application. Refer to config/secret.key");

    HttpServer::new(move || {
        let scope = web::scope("/api")
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello));

        App::new()
            .data(AppState {
                jwt_secret: jwt_secret.clone(),
            })
            .service(scope)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
