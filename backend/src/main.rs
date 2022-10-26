use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use futures::stream::StreamExt;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

const DB_NAME: &str = "zvenigorodok";
const COLL_NAME: &str = "reviews";

#[derive(Debug, Serialize, Deserialize)]
struct Review {
    text: String,
    user: String,
}

#[actix_web::get("/get_reviews")]
async fn get_reviews(client: web::Data<Client>) -> impl Responder {
    let collection: mongodb::Collection<Review> = client.database(DB_NAME).collection(COLL_NAME);
    let cursor = collection.find(None, None).await;
    match cursor {
        Ok(cursor) => {
            let reviews: Vec<Result<Review, _>> = cursor.collect().await;
            let reviews: Vec<Review> = reviews
                .into_iter()
                .collect::<Result<Vec<Review>, _>>()
                .unwrap_or_default();
            HttpResponse::Ok().json(reviews)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[actix_web::post("/add_review")]
async fn add_review(client: web::Data<Client>, req_body: web::Json<Review>) -> impl Responder {
    let collection: mongodb::Collection<Review> = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(req_body.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    env_logger::init();

    let static_directory = std::env::var("STATIC_DIR").unwrap_or(".".into());
    let uri = std::env::var("MONGODB_URI").unwrap_or("mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;

    HttpServer::new(move || {
        let static_directory = static_directory.clone();
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(get_reviews)
            .service(add_review)
            .service(actix_files::Files::new("/", static_directory).index_file("index.html"))
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
