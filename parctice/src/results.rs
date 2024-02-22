/// 1. and  and_then  于之 相反的为 or or_else
/// 2. map:对里面的值在进行一次处理后 map_err:可以进行错误类型转化。
/// 3. map_or:如果是错误，返回默认值，否则返回执行闭包执行的值。
/// 4. map_or_orr :如果ok返回执行结果，错误返回错误的闭包.
/// 5. ok 转换 诚 option
/// 6. unwrap---unwrap_err unwrap_or--unwrap_or_default---unwrap_or_else
#[cfg(test)]
pub mod tests{
    #[test]
    fn test_inspect(){
        let x: u8 = "4"
            .parse::<u8>()
            .inspect(|x| println!("original: {x}"))
            .map(|x| x.pow(3))
            .expect("failed to parse number");
        assert_eq!(x,4)
    }
}