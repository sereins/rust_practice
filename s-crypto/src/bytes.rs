// 适用于网络协议，文件IO，和大量字节操作的场景
// 可以更加高效的操作字节，提升性能。
// 零拷贝的共享内存的。
// 本质上是一个结构体，对内存区域有一个引用,包装了一层。
// BytesMut，Bytes 是内存连续的。 而buf，和buf_mut是不连续的。

#[cfg(test)]
mod tests {
    use bytes::{BufMut, BytesMut};

    #[test]
    fn test() {
        let mut buf = BytesMut::with_capacity(1024);

        buf.put(&b"hello"[..]);
        println!("buf = {:?}", buf);
    }
}
