mod models;
use axum::{
    routing::get,
    Router,
};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use mongodb::{Client, options::ClientOptions};
use std::error::Error;


async fn root() -> &'static str {
    "Hello, world!"
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Загрузка переменных окружения из .env
    dotenv().ok();
    let mongo_username = env::var("MONGO_ADMIN").expect("Mongo admin username should be set");
    let mongo_password = env::var("MONGO_PASSWORD").expect("Mongo admin password should be set");

    let mongo_uri = format!(
        "mongodb+srv://{}:{}@cluster0.oyurklw.mongodb.net/",
        mongo_username,
        mongo_password
    );
    let client_options = ClientOptions::parse(&mongo_uri).await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("mydatabase");
    println!("Успешное подключение к базе данных!");

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World! kfkfkf" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
