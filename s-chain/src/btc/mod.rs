#[cfg(test)]
mod test {
    // use bitcoincore_rpc::{Auth, Client, RpcApi};

    // #[tokio::test]
    // async fn test() {
    //     let rpc_url = "http://nd-713-698-661.p2pify.com/";
    //     let rpc_auth = Auth::UserPass(
    //         "admiring-mcnulty".to_string(),
    //         "boring-gush-siding-specks-strewn-silent".to_string(),
    //     );
    //     let client = Client::new(rpc_url, rpc_auth).unwrap();

    //     let hash = client.get_blockchain_info().unwrap();
    //     println!("Best block hash: {:?}", hash);
    // }

    // https://nd-713-698-661.p2pify.com/06caecb65142b99843ae636f85267218

    // let rpc_url = "https://nd-713-698-661.p2pify.com/06caecb65142b99843ae636f85267218";
    // let rpc_url = "https://nd-202-842-353.p2pify.com/788f110831fe13808302bd79796d55e8";
    // let rpc_auth = Auth::UserPass(
    //     "admiring-mcnulty".to_string(),
    //     "boring-gush-siding-specks-strewn-silent".to_string(),
    // );

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
