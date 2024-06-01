/// 1. and  and_then  于之 相反的为 or or_else
/// 2. map:对里面的值在进行一次处理后 map_err:可以进行错误类型转化。
/// 3. map_or:如果是错误，返回默认值，否则返回执行闭包执行的值。
/// 5. map_or_orr :如果ok返回执行结果，错误返回错误的闭包.
/// 5. ok 转换 诚 option
/// 6. unwrap---unwrap_err unwrap_or--unwrap_or_default---unwrap_or_else
#[cfg(test)]
pub mod tests{
    /// 如果是正确的值进行处理
    #[test]
    fn test_and(){
        let a:Result<i32,&str>  = Ok(3);
        let b:Result<i32,&str>  = Err("hello");

        let c = a.and(b);
        println!("{:?}",c);

        let a:Result<i32,&str>  = Ok(3);
        let d  = a.and_then(|t|Ok(t * 3));
        println!("{:?}",d)
    }

    /// 如果是错误进行处理
    #[test]
    fn test_or(){
        let a:Result<i32,&str>  = Err("first value is err");
        let b:Result<i32,&str>  = Ok(3);

        let c = a.and(b);
        println!("{:?}",c);

        let a:Result<i32,&str>  = Err("is err");
        let d  = a.or_else(|_t|Err("new error"));
        println!("{:?}",d)
    }

    /// 对值或者错误的进一步处理，返回结果还是result类型
    #[test]
    fn test_map(){
        let a:Result<i32,&str>  = Err("error");
        let c = a.map(|t|t * 2);
        println!("{:?}",c);
    }

    /// 对值进一步处理，返回值是解开result后得值
    #[test]
    fn test_map_or(){
        let a:Result<i32,&str>  = Err("error");
        let b = a.map_or(0,|x|{
            x * 2
        });
        println!("map_or result:{:?}",b);

        let c  = a.map_or_else(|_| 2,|t| t * 2);
        println!("map_or_else result:{:?}",c);
    }

    /// 对值的拆开处理,但是回引发panic
    #[test]
    fn test_wrap(){
        let a:Result<i32,&str>  = Err("error");
        let c = a.unwrap();
        println!("unwrap value:{:?}",c);
    }
}