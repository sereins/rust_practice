use secp256k1::{rand, Keypair, Secp256k1, SecretKey};

pub fn generate_private_key() -> SecretKey {
    let secp = Secp256k1::new();
    let keypair = Keypair::new(&secp, &mut rand::thread_rng());
    let key = keypair.secret_key();
    key
}

#[cfg(test)]
mod tests {
    use sha3::{Digest, Keccak256};

    #[test]
    fn test() {
        let secp = super::Secp256k1::new();
        let sk = super::generate_private_key();
        let pk = sk.public_key(&secp);
        println!("public key = {:?}", pk);

        let mut hasher = Keccak256::new();
        hasher.update(pk.to_string());

        let result = hasher.finalize();

        println!("public key hash = {:x?},len = {:?}", result, result.len());

        let mut address = [0x41u8; 21];
        address[1..].copy_from_slice(&result[12..]);

        let addr = bs58::encode(address).with_check().into_string();
        println!("address = {}", addr);
    }
}
