use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use futures::stream::StreamExt;
use mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string;
use mongodb::{bson::DateTime, Client};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
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

    let static_directory = std::env::var("STATIC_DIR").unwrap_or("./public".into());
    let uri = std::env::var("MONGODB_URI").unwrap_or("mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    let is_dev = std::env::var("DEV").is_ok();
    let coll_name = std::env::var("COLL_NAME").unwrap_or("reviews".into());
    let db_name = std::env::var("DB_NAME").unwrap_or("zvenigorodok".into());
    let port = match std::env::var("PORT") {
        Ok(port) => port.parse().unwrap_or(8080),
        _ => 8080,
    };
    let ip = std::env::var("IP").unwrap_or("0.0.0.0".into());
    let collection: mongodb::Collection<Review> = client.database(&db_name).collection(&coll_name);

    let ssl_key = std::env::var("SSL_KEY");
    let ssl_cert = std::env::var("SSL_CERT");

    let server = HttpServer::new(move || {
        let cors = if is_dev {
            Cors::permissive()
        } else {
            Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST"])
        };
        let files = actix_files::Files::new("/", static_directory.clone()).index_file("index.html");

        App::new()
            .app_data(web::Data::new(collection.clone()))
            .service(get_reviews)
            .service(add_review)
            .service(files)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
    });

    if let (Ok(ssl_key), Ok(ssl_cert)) = (ssl_key, ssl_cert) {
        let config = load_rustls_config(&ssl_cert, &ssl_key);
        server
            .bind((ip.clone(), port))?
            .bind_rustls(format!("{ip}:443"), config)?
            .run()
            .await?;
    } else {
        server.bind((ip.clone(), port))?.run().await?;
    }

    Ok(())
}

fn load_rustls_config(cert: &str, key: &str) -> rustls::ServerConfig {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(cert).unwrap());
    let key_file = &mut BufReader::new(File::open(key).unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
