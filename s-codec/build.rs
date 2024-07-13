fn main() {
    prost_build::compile_protos(
        &["src/protobuf/proto/example.proto"],
        &["src/protobuf/proto"],
    )
    .unwrap();
}
