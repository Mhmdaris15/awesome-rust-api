use actix_web::{web, App, HttpServer, Responder};
use mongodb::{Client, options::ClientOptions};

async fn index() -> impl Responder {
    // Connect to MongoDB
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("berufsvernetzen");

    // Perform MongoDB operations here
    // ...

    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

