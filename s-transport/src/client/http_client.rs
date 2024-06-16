use crate::{errors::TransportError, request_builder::ReqBuilder};
use reqwest::header::{self, HeaderMap};

pub struct HttpClient {
    base_url: String,
    client: reqwest::Client,
}

impl HttpClient {
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

    pub fn post(&self, endpoint: &str) -> ReqBuilder {
        let url = format!("{}/{}", self.base_url, endpoint);
        let build = self.client.post(url);
        ReqBuilder(build)
    }

    pub fn get(&self, endpoint: &str) -> ReqBuilder {
        let url = format!("{}/{}", self.base_url, endpoint);
        let build = self.client.get(url);
        ReqBuilder(build)
    }
}
