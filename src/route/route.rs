use mongodb::bson::doc;
use sha2::{Digest, Sha256};

use crate::config::mongodb::mongo_client;
use crate::model::account;

pub async fn login(username: String, password: String) -> &'static str {
    let account_collection = mongo_client().await;
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = format!("{:x}", hasher.finalize());
    println!("{}", result);
    let db = account_collection.database("IntesSocial");
    let collection = db.collection::<account::Account>("account");
    let finder = doc! {
        "$and": [
            doc! {
                "username": username
            },
            doc! {
                "password": result
            }
        ]
    };
    let res = collection.find_one(finder, None).await.unwrap();
    println!("{:?}", res);
    return "Test Login";
}
