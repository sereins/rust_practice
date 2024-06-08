/// 在rust 中有两个库都和secp256k1有关
/// 1. secp256k1 https://docs.rs/secp256k1/latest/secp256k1/ 是对c secp256k1的封装
/// 2. libsecp256k1 https://docs.rs/libsecp256k1/latest/libsecp256k1/ 是纯rust实现的版本
/// 两个都是 对 ECC加密算法，以及 ECDSA 签名的实现，
/// 提供 以下功能
/// 1. 根据一个随机数，创建密钥对
/// 2. 对一段消息 进行 ECDSA 签名
/// 3. 对签名进行验证
///  公钥 没有压缩的时候是65个自己，压缩后是33个自己。
///
///
#[cfg(test)]
mod tests {
    use secp256k1::hashes::{sha256, Hash};
    use secp256k1::rand::rngs::OsRng;
    use secp256k1::{Message, Secp256k1};

    #[test]
    fn test_generate() {
        let secp = Secp256k1::new();
        let (private_key, public_key) = secp.generate_keypair(&mut OsRng);

        println!("private_key:{:?}", private_key.secret_bytes());
        println!(
            "public_key:{:?},public_key_len = {}",
            public_key.to_string(),
            public_key.to_string().len()
        );
    }

    #[test]
    fn test_sign() {
        let secp = Secp256k1::new();
        let (private_key, _public_key) = secp.generate_keypair(&mut OsRng);

        let digest = sha256::Hash::hash("hello".as_bytes());
        let message = Message::from_digest(digest.to_byte_array());

        let sig = secp.sign_ecdsa(&message, &private_key);

        println!("sig:{:?}", hex::encode(sig.serialize_compact()));
    }
}
