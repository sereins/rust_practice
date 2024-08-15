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

    // 可执行的taproot 多签脚本
    // #[tokio::test]
    // async fn test_other() {
    //     println!("Generating a new taproot transaction");
    //
    //     let secp = Secp256k1::new();
    //     let alice_secret =
    //         SecretKey::from_str("2bd806c97f0e00af1a1fc3328fa763a9269723c8db8fac4f93af71db186d6e90")
    //             .unwrap();
    //     let bob_secret =
    //         SecretKey::from_str("81b637d8fcd2c6da6359e6963113a1170de795e4b725b84d1e0b4cfd9ec58ce9")
    //             .unwrap();
    //     let charlie_secret =
    //         SecretKey::from_str("81b637d8fcd2c6da6359e6963113a1170de795e4b725b84d1e0b4cfd9ec58ce8")
    //             .unwrap();
    //     let internal_secret =
    //         SecretKey::from_str("1229101a0fcf2104e8808dab35661134aa5903867d44deb73ce1c7e4eb925be8")
    //             .unwrap();
    //
    //     let alice = Keypair::from_secret_key(&secp, &alice_secret);
    //     let bob = Keypair::from_secret_key(&secp, &bob_secret);
    //     let charlie = Keypair::from_secret_key(&secp, &charlie_secret);
    //
    //     let internal = Keypair::from_secret_key(&secp, &internal_secret);
    //
    //     println!("alice public key {}", alice.public_key());
    //     println!("bob public key {}", bob.public_key());
    //     println!("internal public key {}", internal.public_key());
    //
    //     // 2 -of -3 signer script
    //     // 2 -of -3 signer script
    //     // let wallet_script = Builder::new()
    //     //     .push_x_only_key(&bob.public_key().into())
    //     //     .push_opcode(all::OP_CHECKSIGVERIFY) // Bob key is necessary
    //     //     .push_x_only_key(&alice.public_key().into())
    //     //     .push_opcode(all::OP_CHECKSIG)
    //     //     .push_x_only_key(&charlie.public_key().into())
    //     //     .push_opcode(all::OP_CHECKSIGADD)
    //     //     .push_int(1) // since the number we are comparing is the valid sigs from alice and charlie, it should be 1
    //     //     .push_opcode(all::OP_GREATERTHANOREQUAL) // atleast one sig
    //     //     .into_script();
    //
    //     let wallet_script = Builder::new()
    //         .push_x_only_key(&bob.public_key().into())
    //         .push_opcode(all::OP_CHECKSIG)
    //         .push_x_only_key(&alice.public_key().into())
    //         .push_opcode(OP_CHECKSIGADD)
    //         .push_x_only_key(&charlie.public_key().into())
    //         .push_opcode(OP_CHECKSIGADD)
    //         .push_opcode(OP_PUSHNUM_2)
    //         .push_opcode(OP_GREATERTHANOREQUAL)
    //         .into_script();
    //
    //     println!("Script {:?}", wallet_script);
    //
    //     let builder = TaprootBuilder::with_huffman_tree(vec![(1, wallet_script.clone())]).unwrap();
    //     let tap_info = builder
    //         .finalize(&secp, internal.public_key().into())
    //         .unwrap();
    //
    //     let address = Address::p2tr(
    //         &secp,
    //         tap_info.internal_key(),
    //         tap_info.merkle_root(),
    //         bitcoin::Network::Regtest,
    //     );
    //     println!("Taproot wallet address {:?}", address);
    //
    //     // let tx_id =
    //     //     Txid::from_str("23573e308070b33aa4a1ddad5d380118163603bcef0348707dc525c6ea267b9d").unwrap();
    //     let tx_id =
    //         Txid::from_str("3e7a8ea020b359b2364e20537e240b42234805a22f8a02ef60f97a87dd0c47f8").unwrap();
    //     let vec_tx_in: Vec<_> = vec![TxIn {
    //         previous_output: bitcoin::OutPoint {
    //             txid: tx_id,
    //             vout: 0,
    //         },
    //         script_sig: ScriptBuf::default(),
    //         sequence: bitcoin::Sequence(0xFFFFFFFF),
    //         witness: Witness::default(),
    //     }];
    //
    //     println!("Found UTXOS {:?}", vec_tx_in);
    //
    //     let to = Address::from_str("bcrt1qavm69e9xuqme0w0x752sl2lj5zjw2e637eshgc")
    //         .unwrap()
    //         .require_network(bitcoin::network::Network::Regtest)
    //         .unwrap();
    //
    //     // 构建交易
    //     let mut tx = Transaction {
    //         version: Version(2),
    //         lock_time: absolute::LockTime::ZERO,
    //         input: vec![TxIn {
    //             previous_output: vec_tx_in[0].previous_output.clone(),
    //             script_sig: ScriptBuf::default(),
    //             sequence: bitcoin::Sequence(0xFFFFFFFF),
    //             witness: Witness::default(),
    //         }],
    //         output: vec![TxOut {
    //             value: Amount::from_int_btc(9),
    //             script_pubkey: to.script_pubkey(),
    //         }],
    //     };
    //
    //     // let script_pubkey =
    //     //     ScriptBuf::from_hex("512058665fca94b0a1f55b5e65ac6e74d6bc3f4ff6432fdab17facba63af23b2e707")
    //     //         .unwrap();
    //
    //     let script_pubkey =
    //         ScriptBuf::from_hex("5120ca410b97d62153e66498750d5e15207fa6480477d911939dae0594005b915b78")
    //             .unwrap();
    //
    //     let pre_out = vec![TxOut {
    //         value: Amount::from_int_btc(10),
    //         script_pubkey: script_pubkey,
    //     }];
    //     let prevouts = Prevouts::All(&pre_out);
    //
    //     let sighash_sig = SighashCache::new(&mut tx.clone())
    //         .taproot_script_spend_signature_hash(
    //             0,
    //             &prevouts,
    //             ScriptPath::with_defaults(&wallet_script),
    //             TapSighashType::Default,
    //         )
    //         .unwrap();
    //
    //     let actual_control = tap_info
    //         .control_block(&(wallet_script.clone(), LeafVersion::TapScript))
    //         .unwrap();
    //
    //     let msg = Message::from(sighash_sig);
    //     let sig1 = secp.sign_schnorr(&msg, &alice);
    //     let sig2 = secp.sign_schnorr(&msg, &bob);
    //     let sig3 = secp.sign_schnorr(&msg, &charlie);
    //
    //     let schnorr_sig1 = bitcoin::taproot::Signature {
    //         signature: sig1,
    //         sighash_type: TapSighashType::Default,
    //     };
    //
    //     let schnorr_sig2 = bitcoin::taproot::Signature {
    //         signature: sig2,
    //         sighash_type: TapSighashType::Default,
    //     };
    //
    //     let _schnorr_sig3 = bitcoin::taproot::Signature {
    //         signature: sig3,
    //         sighash_type: TapSighashType::Default,
    //     };
    //
    //     let mut wit = Witness::new();
    //
    //     wit.push(vec![]);
    //     // wit.push(schnorr_sig3.to_vec());
    //     wit.push(schnorr_sig1.to_vec());
    //     wit.push(schnorr_sig2.to_vec());
    //
    //     // wit.push(schnorr_sig1.to_vec());
    //     // wit.push(schnorr_sig2.to_vec());
    //     // wit.push(schnorr_sig3.to_vec());
    //
    //     wit.push(wallet_script.to_bytes());
    //     wit.push(actual_control.serialize());
    //     tx.input[0].witness = wit.clone();
    //
    //     println!("Final transaction {:?}", tx);
    //     let hex_raw = consensus::encode::serialize_hex(&tx);
    //     println!("Final hex {:?}", hex_raw);
    // }
}
