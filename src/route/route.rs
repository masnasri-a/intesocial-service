use mongodb::bson::doc;
use sha2::{Digest, Sha256};

use crate::config::mongodb::mongo_client;
use crate::model::account;
use crate::util::auth;

pub async fn login(username: String, password: String) -> String {
    let mut temp_username = username;

    let account_collection = mongo_client().await;
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = format!("{:x}", hasher.finalize());
    println!("{}", result);

    let token = auth::create_jwt(&mut temp_username);

    let db = account_collection.database("IntesSocial");
    let collection = db.collection::<account::Account>("account");
    let finder = doc! {
        "$and": [
            doc! {
                "username": temp_username
            },
            doc! {
                "password": result
            }
        ]
    };
    let res = collection.find_one(finder, None).await.unwrap();
    match res {
        Some(doc)=>{
            println!("{:?}",doc);
            return token;
        }, None=>{
            return "Not Found".to_string();
        }
    }
}
