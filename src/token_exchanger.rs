use async_trait::async_trait;
use hyper::{Body, http, Request};
use serde::{Deserialize, Serialize};

/// Provides an async trait for conditionally exchanging tokens with a token exchange service.
#[async_trait]
pub trait TokenExchanger {
    async fn maybe_exchange(
        &self,
        request: &Request<Body>
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

/// Response from a [TokenExchanger] upon successfully completing a token exchange.
/// This response includes an `access_token` and `token_type`, which will be used to
/// populate the `Authorization` header on the proxied request.
#[derive(Serialize,Deserialize,Debug)]
pub struct TokenExchangeResponse {
    /// The token to forward to the upstream service.
    pub access_token: String,
    /// The token type (e.g. `Bearer`).
    pub token_type: String
}
