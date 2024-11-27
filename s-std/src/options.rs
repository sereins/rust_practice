/// 1. and,and_then <==> or,or_else
/// 2. map:对里面的值在进行一次处理后 map_err:可以进行错误类型转化。
/// 3. map_or: 对Some进行计算，否则返回默认 map_or_else
/// 5. ok_or,ok_or_else 转换成 Result类型
/// 6. take 获取Option里面的值
/// 6. unwrap---unwrap_err unwrap_or--unwrap_or_default---unwrap_or_else
#[cfg(test)]
mod tests {
    #[test]
    fn test_and() {
        let a: Option<i32> = Some(3);
        let b: Option<i32> = None;

        let c = a.and(b);
        println!("{:?}", c);

        let a: Option<i32> = Some(3);
        let d = a.and_then(|t| Some(t * 3));
        println!("{:?}", d)
    }

    /// 如果是None进行处理
    #[test]
    fn test_or() {
        let a: Option<i32> = Some(3);
        let b: Option<i32> = None;

        let c = a.or(b);
        println!("{:?}", c);

        let a: Option<i32> = Some(3);
        let d = a.or_else(|| Some(3));
        println!("{:?}", d)
    }

    /// 对值或者错误的进一步处理，返回结果还是Option类型
    #[test]
    fn test_map() {
        let a: Option<i32> = Some(3);
        let c = a.map(|t| t * 2);
        println!("{:?}", c);
    }

    /// 对值进一步处理，返回值是解开result后得值
    #[test]
    fn test_map_or() {
        let a: Option<i32> = Some(3);
        let b = a.map_or(0, |x| x * 2);
        println!("map_or result:{:?}", b);

        let c = a.map_or_else(|| 2, |t| t * 2);
        println!("map_or_else result:{:?}", c);
    }

    /// 对值的拆开处理,但是回引发panic
    #[test]
    fn test_wrap() {
        let a: Option<i32> = Some(3);
        let c = a.unwrap();
        println!("unwrap value:{:?}", c);
    }
}
