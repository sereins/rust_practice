use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Serialize)]
pub struct JsonRpcPrams<T> {
    pub method: String,
    pub jsonrpc: String,
    pub id: u32,
    pub params: Option<T>,
}

impl<T: Serialize + Debug> JsonRpcPrams<T> {
    pub fn default() -> Self {
        Self {
            method: "".to_owned(),
            jsonrpc: "2.0".to_owned(),
            id: 1,
            params: None,
        }
    }

    pub fn set_params(mut self, p: T) -> Self {
        self.params = Some(p);
        self
    }
    pub fn method(mut self, m: &str) -> Self {
        self.method = m.to_owned();
        self
    }
}
