use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use futures::stream::StreamExt;
use mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string;
use mongodb::{bson::DateTime, Client};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Review {
    text: String,
    user: String,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    date: DateTime,
}

#[actix_web::get("/get_reviews")]
async fn get_reviews(collection: web::Data<mongodb::Collection<Review>>) -> impl Responder {
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
async fn add_review(
    collection: web::Data<mongodb::Collection<Review>>,
    req_body: web::Json<Review>,
) -> impl Responder {
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
    let is_dev = std::env::var("DEV").is_ok();
    let coll_name = std::env::var("COLL_NAME").unwrap_or("reviews".into());
    let db_name = std::env::var("DB_NAME").unwrap_or("zvenigorodok".into());
    let collection: mongodb::Collection<Review> = client.database(&db_name).collection(&coll_name);

    HttpServer::new(move || {
        let cors = if is_dev {
            Cors::permissive()
        } else {
            Cors::default()
        };
        let files = actix_files::Files::new("/", static_directory.clone()).index_file("index.html");

        App::new()
            .app_data(web::Data::new(collection.clone()))
            .service(get_reviews)
            .service(add_review)
            .service(files)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
