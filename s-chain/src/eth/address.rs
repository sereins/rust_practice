#[cfg(test)]
mod tests {
    use secp256k1::{rand, Secp256k1, SecretKey};
    use sha3::{Digest, Keccak256};

    // 1. 根据一个随机数，创建密钥对
    // 2. 获得公钥，并对公钥进行keccak256 hash

    #[test]
    fn test_addr() {
        let secret_key = SecretKey::new(&mut rand::thread_rng());

        let secp = Secp256k1::new();
        let pubkey = secret_key.public_key(&secp);

        let pubkey_bytes = pubkey.serialize_uncompressed();

        let mut hasher = Keccak256::new();
        hasher.update(&pubkey_bytes[1..]);

        let key_hash = hasher.finalize();
        let addr = key_hash[12..].to_vec();

        println!("addr: {:?}", format!("0x{}", hex::encode(addr)));
    }
}
