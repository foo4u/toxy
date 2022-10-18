use serde::Deserialize;

/// Upstream, reverse proxy configuration.
#[derive(Deserialize)]
pub struct UpstreamConfig {
    #[serde(alias = "forward-url")]
    pub forward_url: String
}
