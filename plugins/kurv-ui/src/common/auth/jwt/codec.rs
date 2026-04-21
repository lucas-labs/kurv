use {
    crate::common::auth::err::AuthError,
    chrono::{Duration, Utc},
    jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, TokenData, Validation, decode},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub iat: u64,
    pub exp: u64,
    pub sub: String,
}

#[derive(Clone)]
pub struct JwtCodec {
    pub decoding_secret: DecodingKey,
    pub encoding_secret: EncodingKey,
    pub validation: Validation,
}

impl JwtCodec {
    pub fn new(secret: &str) -> Self {
        Self {
            decoding_secret: DecodingKey::from_secret(secret.as_ref()),
            encoding_secret: EncodingKey::from_secret(secret.as_ref()),
            validation: Validation::new(Algorithm::HS256),
        }
    }

    pub async fn decode(&self, token: &str) -> Result<TokenData<Claims>, AuthError> {
        match decode::<Claims>(token, &self.decoding_secret, &self.validation) {
            Ok(token_data) => Ok(token_data),
            Err(err) => Err(err.into()),
        }
    }

    pub async fn create_token(&self, sub: &str) -> Result<String, AuthError> {
        let claims = Claims {
            iat: Utc::now().timestamp() as u64,
            exp: (Utc::now() + Duration::hours(1)).timestamp() as u64,
            sub: sub.to_string(),
        };

        let header = jsonwebtoken::Header::new(Algorithm::HS256);
        match jsonwebtoken::encode(&header, &claims, &self.encoding_secret) {
            Ok(token) => Ok(token),
            Err(_) => Err(AuthError::InternalError),
        }
    }
}
