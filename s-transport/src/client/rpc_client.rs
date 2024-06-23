use crate::{errors::TransportError, request_builder::ReqBuilder};
use reqwest::header::{self, HeaderMap};
use serde::Serialize;
use std::fmt::Debug;

pub struct RpcClient {
    base_url: String,
    client: reqwest::Client,
}

impl RpcClient {
    pub fn new(base_url: &str) -> Result<Self, TransportError> {
        let mut headers = HeaderMap::new();

        headers.append(header::ACCEPT, "application/json".parse().unwrap());
        headers.append(header::CONTENT_TYPE, "application/json".parse().unwrap());

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(|e| TransportError::ReqError(e))?;

        Ok(Self {
            base_url: base_url.to_owned(),
            client,
        })
    }

    pub fn set_params<T: Serialize + Debug>(&self, p: T) -> ReqBuilder {
        tracing::info!("rpc params: {:?}", p);
        let build = self.client.post(self.base_url.clone()).json(&p);
        ReqBuilder(build)
    }
}
