use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: i64,
    exp: i64,
}

pub fn create_jwt(username: &mut str) -> String {
    let header = Header::new(Algorithm::HS256);
    let claims = Claims {
        sub: username.to_owned(),
        iat: Utc::now().timestamp(),
        exp: (Utc::now() + Duration::hours(1)).timestamp(),
    };
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret("nasriganteng".as_ref()),
    )
    .unwrap();
    return token;
    // let data = TokenData::<Claims>::from(&header, &claims, &key)?;
    // Ok(data)
    // let result = jsonwebtoken::encode(&header, data, &key);
    // return result;
}
