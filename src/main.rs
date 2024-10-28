use actix_web::{
    web::{self},
    App, HttpServer, Responder,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct Register {
    _username: Option<String>,
    _password: Option<String>,
    email: String,
}

async fn welcome() -> impl Responder {
    "Welcome to Auto Api! v1"
}

async fn login(user: web::Query<Login>) -> impl Responder {
    format!(
        "Hello, {} with username: {} and password: {}",
        user.username, user.username, user.password
    )
}

async fn register(user: web::Query<Register>) -> impl Responder {
    format!("Registered sucessfully for email {}", user.email)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .route("/", web::get().to(welcome))
                .route("/register", web::get().to(register))
                .route("/login", web::get().to(login)),
        )
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
