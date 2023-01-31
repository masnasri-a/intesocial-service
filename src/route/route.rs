use chrono::Utc;
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
        Some(doc) => {
            println!("{:?}", doc);
            return token;
        }
        None => {
            return "Not Found".to_string();
        }
    }
}

pub async fn register(username: &str, full_name: &str, email: &str, password: &str) -> String {
    let account_collection = mongo_client().await;
    let db = account_collection.database("IntesSocial");
    let collection = db.collection::<account::Account>("account");
    let finder = doc! {
        "username": &username
    };
    let res = collection.find_one(finder, None).await.unwrap();
    match res {
        Some(_doc) => {
            return "Username Already Used!".to_string();
        }
        None => {
            let mut hasher = Sha256::new();
            hasher.update(username.as_bytes());
            let result_username = format!("{:x}", hasher.finalize());

            let mut hasher_password = Sha256::new();
            hasher_password.update(password.as_bytes());
            let result = format!("{:x}", hasher_password.finalize());

            let model_register = account::Account {
                _id: result_username,
                username: username.to_owned(),
                full_name: full_name.to_owned(),
                email: email.to_owned(),
                password: result,
                created_at: Utc::now().timestamp_millis() as u64,
            };
            println!("{:?}", &model_register);
            let _res = collection.insert_one(model_register, None).await;

            return "Account has been created".to_string();
        }
    }
}
