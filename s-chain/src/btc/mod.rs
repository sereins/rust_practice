#[cfg(test)]
mod test {
    use reqwest::Client;
    use serde_json::json;

    #[tokio::test]
    async fn test() {
        let rpc_url = "https://nd-713-698-661.p2pify.com/";
        let username = "admiring-mcnulty";
        let password = "boring-gush-siding-specks-strewn-silent";

        let client = Client::new();

        // 创建请求的 JSON-RPC 数据
        let request_body = json!({
            "jsonrpc": "1.0",
            "id": "curltest",
            "method": "getblockchaininfo",
            "params": []
        });

        // 发送 HTTP POST 请求
        let response = client
            .post(rpc_url)
            .basic_auth(username, Some(password))
            .json(&request_body)
            .send()
            .await
            .unwrap();

        // 解析 JSON 响应
        let response_json: serde_json::Value = response.json().await.unwrap();

        // 打印响应
        println!("Response: {:#?}", response_json);
    }
}
