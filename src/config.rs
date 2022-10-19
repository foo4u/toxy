use serde::Deserialize;

/// Upstream, reverse proxy configuration.
#[derive(Deserialize)]
pub struct UpstreamConfig {
    /// The upstream HTTP server URL this proxy should forward requests to after potentially
    /// initiating a token exchange.
    #[serde(alias = "forward-url")]
    pub forward_url: String
}
