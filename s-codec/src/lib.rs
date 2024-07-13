// about decode and encode rust data type
pub mod protobuf;

pub mod proto {
    pub mod example {
        include!(concat!(env!("OUT_DIR"), "/proto.example.rs"));
    }
}

#[cfg(test)]
mod tests {
    use prost::Message;

    use crate::proto::example::Example;

    #[test]
    fn tests() {
        let example = Example {
            id: 3,
            name: "test".to_string(),
        };

        // 将数据编码为字
        let example_bytes = example.encode_to_vec();

        // 从字节中提取数据
        let example2 = Example::decode(example_bytes.as_slice()).unwrap();

        println!("example {:?}", example_bytes);
        println!("example {:?}", example2);
    }
}
