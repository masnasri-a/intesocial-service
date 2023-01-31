use dotenv::dotenv;
use mongodb::{Client, options::ClientOptions};

pub async fn mongo_client() -> mongodb::Client{
    dotenv().ok();
    let mongo_uri = std::env::var("MONGO_URI").expect("mongodb://admin:admi@localhost:27017/");
    let mut client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    client_options.app_name = Some("My App".to_string());
    let client = Client::with_options(client_options).unwrap();
    return client;
}
