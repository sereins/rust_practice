use crate::errors::TransportError;
use reqwest::RequestBuilder;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub struct ReqBuilder(pub RequestBuilder);

impl ReqBuilder {
    pub fn json(mut self, v: impl Serialize + Debug) -> Self {
        tracing::info!("request params: {:?}", v);
        self.0 = self.0.json(&v);
        self
    }

    pub async fn send<T: DeserializeOwned>(self) -> Result<String, crate::errors::TransportError> {
        let res = self
            .0
            .send()
            .await
            .map_err(|e| TransportError::ReqError(e))?;

        let response_str = res.text().await.map_err(|e| TransportError::ReqError(e))?;

        Ok(response_str)
        // tracing::info!("response: {}", response_str);
        // serde_json::from_str::<T>(&response_str).map_err(|e| TransportError::SerdeError(e))
        // res.json::<T>()
        //     .await
        //     .map_err(|e| TransportError::ReqError(e))
    }
}
