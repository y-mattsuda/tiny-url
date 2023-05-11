use actix_web::{App, HttpServer};
use driver::routes::health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");
    HttpServer::new(|| App::new().service(health::health).service(health::index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
