use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use std::error::Error;

#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
