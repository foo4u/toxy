use async_trait::async_trait;
use hyper::{Body, http, Request};
use serde::{Deserialize, Serialize};

/// Provides an async trait for conditionally exchanging tokens with a token exchange service.
#[async_trait]
pub trait TokenExchanger {
    async fn maybe_exchange(
        &self,
        request: &Box<Request<Body>>
    ) -> Result<Option<TokenExchangeResponse>, TokenExchangeError>;
}

/// Error returned for token exchange failures.
#[derive(Debug, Clone)]
pub struct TokenExchangeError {
    pub msg: String,
    pub status_code: Option<http::StatusCode>
}

/// Token exchange error implementation.
impl TokenExchangeError {
    pub fn from_error(err: reqwest::Error) -> TokenExchangeError {
        TokenExchangeError {
            msg: err.to_string(),
            status_code: err.status()
        }
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct TokenExchangeResponse {
    pub access_token: String,
    pub token_type: String
}