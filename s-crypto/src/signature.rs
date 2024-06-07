#[cfg(test)]
mod tests {
    use secp256k1::{Secp256k1, SecretKey};

    #[test]
    fn test() {
        let private_key_hex = "2e8b5599ed26d8b465d66049c42df32df744baf4bcf18f777d2daf6b28e06e5b";
        let private_key_bytes = hex::decode(private_key_hex).unwrap();
        let private_key = SecretKey::from_slice(&private_key_bytes).unwrap();

        let hash = "2359f1c41ed99d25f90b60d6a451376c95b87108928c3de8ed0a3fbdb11afa59";
        let hash = hex::decode(hash).unwrap();

        let msg = secp256k1::Message::from_digest_slice(&hash).unwrap();

        let secp = Secp256k1::new();
        let c = secp.sign_ecdsa_recoverable(&msg, &private_key);

        let (id, sign) = c.serialize_compact();

        println!("id: {:?}, sign: {:?}", id, hex::encode(sign));
    }
}
