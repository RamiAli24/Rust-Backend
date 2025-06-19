use chrono::{Duration, Utc};
use forge_api_db::entities::users::User;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AuthUser {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    name: String,
    exp: i64,
}

pub async fn get_jwt(user: User) -> Result<String, String> {
    // // 1. Lookup user by name
    // let user = find_user_by_name(name, &app_state.db_pool)
    //     .await
    //     .ok_or_else(|| "Invalid credentials".to_string())?;

    // // 2. Verify password
    // let password_ok =
    //     verify(pass, &user.hashed_pass).map_err(|_| "Invalid credentials".to_string())?;

    // if !password_ok {
    //     return Err("Invalid credentials".to_string());
    // }
    let token = encode(
        &Header::default(),
        &Claims {
            name: user.name,
            exp: (Utc::now() + Duration::minutes(2)).timestamp(),
        },
        &EncodingKey::from_secret("dummy_secret_key".as_bytes()),
    )
    .map_err(|e| e.to_string());

    return token;
}

pub fn decode_jwt(token: &str) -> Result<User, String> {
    let token_data = decode::<User>(
        token,
        &DecodingKey::from_secret("dummy_secret_key".as_bytes()),
        &Validation::default(),
    );

    match token_data {
        Ok(token_data) => Ok(token_data.claims),

        Err(e) => Err(e.to_string()),
    }
}
