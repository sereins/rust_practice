// 使用sha2 对数据进行hash处理
// 接受一个字节的数组，让后将字节数组进行finalize处理
#[cfg(test)]
mod tests {
    use sha2::Digest;
    use sha2::Sha256;

    #[test]
    fn test_sha256() {
        let mut hasher = Sha256::new();
        let str = b"hello world";

        // sha2-256 算法
        hasher.update(str);
        let hash = hasher.finalize();

        println!("sha2-256 hash result:{}", hex::encode(&hash));
    }

    #[test]
    fn test_sha3_256() {
        use sha3::Digest;
        use sha3::Keccak256;
        use sha3::Sha3_256;

        // sha3-256
        let mut hasher = Sha3_256::new();
        hasher.update(b"hello world");
        let hash = hasher.finalize();

        println!("sha3-256 hash result:{}", hex::encode(&hash));

        // keccak256
        let mut hasher = Keccak256::new();
        hasher.update("hello world".as_bytes());
        let hash = hasher.finalize();

        println!("keccak256 hash result:{}", hex::encode(&hash));
    }
}
