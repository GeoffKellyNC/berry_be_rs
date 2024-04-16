use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct JwtConfig {
    secret: String,
    algorithm: Algorithm,
}

pub enum JwtAlgorithm {
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
    ES256,
    ES384,
    ES512,
}

impl From<JwtAlgorithm> for Algorithm {
    fn from(algorithm: JwtAlgorithm) -> Self {
        match algorithm {
            JwtAlgorithm::HS256 => Algorithm::HS256,
            JwtAlgorithm::HS384 => Algorithm::HS384,
            JwtAlgorithm::HS512 => Algorithm::HS512,
            JwtAlgorithm::RS256 => Algorithm::RS256,
            JwtAlgorithm::RS384 => Algorithm::RS384,
            JwtAlgorithm::RS512 => Algorithm::RS512,
            JwtAlgorithm::ES256 => Algorithm::ES256,
            JwtAlgorithm::ES384 => Algorithm::ES384,
            _ => panic!("JWT Algorithm not supported"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub unxid: String,
    pub exp: usize,
    pub twitch_access_token: String,
    pub twitch_id: String,
}

#[derive(Debug)]
pub enum JwtError {
    TokenCreationError(jsonwebtoken::errors::Error),
    TokenValidationError(jsonwebtoken::errors::Error),
    ConfigurationError(String),
}

impl JwtConfig {
    pub fn new(algorithm: Algorithm) -> Self {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        Self { secret, algorithm }
    }

    pub fn generate_token(&self, claims: &Claims) -> Result<String, JwtError> {
        encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(JwtError::TokenCreationError)
    }

    pub fn validate_token(&self, token: &str) -> Result<jsonwebtoken::TokenData<Claims>, JwtError> {
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::new(self.algorithm),
        ) {
            Ok(token_data) => Ok(token_data),
            Err(err) => {
                match err.kind() {
                    jsonwebtoken::errors::ErrorKind::InvalidToken => Err(JwtError::TokenValidationError(err)),
                    jsonwebtoken::errors::ErrorKind::InvalidIssuer => Err(JwtError::TokenValidationError(err)),
                    jsonwebtoken::errors::ErrorKind::InvalidAudience => Err(JwtError::TokenValidationError(err)),
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => Err(JwtError::TokenValidationError(err)),
                    jsonwebtoken::errors::ErrorKind::InvalidAlgorithm => Err(JwtError::TokenValidationError(err)),
                    _ => Err(JwtError::TokenValidationError(err)),
                }
            }
        }
    }
    pub fn generate_exp_time(days: usize) -> usize {
        let now = chrono::Utc::now().timestamp();
        now as usize + (days * 24 * 60 * 60)
    }
}
