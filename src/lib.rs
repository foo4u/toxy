#![doc = include_str!("../README.md")]

pub mod config;
pub mod token_exchanger;

use std::convert::Infallible;
use std::net::{IpAddr};
use hyper::{Body, Request, Response, StatusCode};
use hyper::header::AUTHORIZATION;
use hyper::http::HeaderValue;
use crate::config::UpstreamConfig;
use crate::token_exchanger::TokenExchanger;


/// Provides a token exchange aware reverse proxy.
pub struct Toxy {
    config: UpstreamConfig,
    token_exchanger: Box<dyn TokenExchanger + Sync + Send>
}

impl Toxy {
    /// Creates a new [Toxy] with the given configuration and [TokenExchanger]
    ///
    pub fn with_token_exchanger(config: UpstreamConfig, token_exchanger: Box<dyn TokenExchanger + Sync + Send>) -> Toxy {
        Toxy {
            config,
            token_exchanger
        }
    }

    pub async fn handle(&self, client_ip: IpAddr, mut req: Box<Request<Body>>) -> Result<Response<Body>, Infallible> {
        let tox = self.token_exchanger.maybe_exchange(&req).await;

        match tox {
            Ok(response) => {
                match response {
                    Some(tox_res) => {
                        let val = format!("{} {}", tox_res.token_type, tox_res.access_token);
                        req.headers_mut().remove(AUTHORIZATION);
                        req.headers_mut().insert(AUTHORIZATION, HeaderValue::from_str(val.as_str()).unwrap());
                    }
                    _ => {
                        tracing::debug!("Skipped token exchange for: {} {}", &req.method(), &req.uri())
                    }
                }
            }
            Err(error) => {
                tracing::error!("Token exchange failed: {:?}", error);
                return match error.status_code {
                    Some(status) => {
                        Ok(Response::builder()
                            .status(status)
                            .body(Body::empty())
                            .unwrap())
                    }
                    None => {
                        return self.do_exchange(client_ip, *req).await;
                    }
                }
            }
        }

        self.do_exchange(client_ip, *req).await
    }

    async fn do_exchange(&self, client_ip: IpAddr, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        match hyper_reverse_proxy::call(client_ip, &*self.config.forward_url, req).await {
            Ok(response) => { Ok(response) }
            Err(error) => {
                tracing::error!("Proxy request failed: {:?}", error);
                Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::empty())
                    .unwrap())
            }
        }
    }
}
