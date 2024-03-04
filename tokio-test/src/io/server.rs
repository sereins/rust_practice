// #[tokio::main]
// async fn main() {
//     println!("hello world");
// }

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn server() {
        println!("hello world");
    }
}
