use s_transport::{client::rpc_client::RpcClient, types::JsonRpcPrams};
use serde_json::json;

#[allow(dead_code)]
struct Provider {
    client: RpcClient,
}

#[allow(dead_code)]
impl Provider {
    pub fn new(url: &str) -> Self {
        let client = RpcClient::new(url).unwrap();
        Self { client }
    }
    pub async fn balance(&self, addr: &str) {
        let params = JsonRpcPrams::default()
            .method("getBalance")
            .set_params(vec![addr]);

        let res = self
            .client
            .set_params(params)
            .send::<String>()
            .await
            .unwrap();

        println!("res: {}", res);
    }

    pub async fn token_balance(&self, addr: &str) {
        let req = vec![
            addr.into(),
            json!({
                "mint": "C49WUif5gXpCyHqv391VUZB6E9QRfQrF7CGnyFVtwbAB",
            }),
            json!({
                "encoding": "jsonParsed"
            }),
        ];
        let params = JsonRpcPrams::default()
            .method("getTokenAccountsByOwner")
            .set_params(req);

        let res = self
            .client
            .set_params(params)
            .send::<String>()
            .await
            .unwrap();

        println!("res: {}", res);
    }
}

#[cfg(test)]
mod tests {
    use super::Provider;
    use s_tools::log::init_log;

    fn get_provider() -> Provider {
        init_log();

        let url = "https://api.devnet.solana.com";
        Provider::new(url)
    }

    #[tokio::test]
    async fn test_balance() {
        let provider = get_provider();

        let addr = "Bry8Z2CPQmNtAfU9weyje3WQsV112jUbTm58NYY8DpGL";
        provider.balance(addr).await;
    }

    #[tokio::test]
    async fn token_balance() {
        let provider = get_provider();

        // let addr = "Bry8Z2CPQmNtAfU9weyje3WQsV112jUbTm58NYY8DpGL";
        let addr = "GE93MHXVvnsbhxu7Ttpp7zTiipJeCX3QFXueSK2dCJe6";
        provider.token_balance(addr).await;
    }
}
