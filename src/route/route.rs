use mongodb::bson::doc;

use crate::config::mongodb::mongo_client;
use crate::model::account;

pub async fn login()-> &'static str {
    let account_collection = mongo_client().await;
    let _filter = doc! { "username": "masnasri"};
    let db = account_collection.database("IntesSocial");
    let collection = db.collection::<account::Account>("account");
    let finder = doc! {"username":"nasnas"};
    let res = collection.find_one(finder, None).await.unwrap();
    println!("{:?}",res);
    return "Test Login";
}