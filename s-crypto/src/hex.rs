// 对16进制的数据进行 vec u8类型的转换
// decode :将16进制的数据转换为vec u8
// encode :将vec u8类型的数据转换为16进制的数据

#[cfg(test)]
mod tests {
    #[test]
    fn test_decode() {
        let str = "2e8b5599ed26d8b465d66049c42df32df744baf4bcf18f777d2daf6b28e06e5b   ";
        let bytes = hex::decode(str).unwrap();

        // 两个16进制数占用了1个字节
        println!("str  len = : {:?}", str.len());
        println!("bytes len = : {:?}", bytes.len());

        println!("bytes: {:?}", bytes);
    }

    #[test]
    fn test_encode() {
        let bytes = hex::decode("2e8b5599ed26d8b465d66049c42df32df744baf4bcf18f777d2daf6b28e06e5b")
            .unwrap();
        let hex = hex::encode(bytes);
        println!("hex: {:?}", hex);
    }
}
