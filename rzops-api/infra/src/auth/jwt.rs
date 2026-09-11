use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use uuid::Uuid;

use rzops_domain::models::claims::Claims;
use rzops_domain::ports::TokenService;

/// JWT service for creating and validating tokens.
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration_seconds: u64,
}

impl JwtService {
    pub fn new(secret: &[u8], expiration_seconds: u64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
            expiration_seconds,
        }
    }

    /// Create a JWT token for the given user.
    pub fn create_token(&self, user_id: Uuid, email: &str, is_superuser: bool) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now().timestamp() as u64;
        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            is_superuser,
            exp: now + self.expiration_seconds,
            iat: now,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
    }

    /// Validate a JWT token and return the claims.
    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}

/// Implement the domain TokenService port for JwtService.
impl TokenService for JwtService {
    fn create_token(&self, user_id: Uuid, email: &str, is_superuser: bool) -> Result<String, String> {
        self.create_token(user_id, email, is_superuser)
            .map_err(|e| e.to_string())
    }

    fn validate_token(&self, token: &str) -> Result<Claims, String> {
        self.validate_token(token)
            .map_err(|e| e.to_string())
    }
}
