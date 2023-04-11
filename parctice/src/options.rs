

#[cfg(test)]
mod tests{
    #[test]
    fn t_and(){
        // and 方法:如果参数是none 返回none,如果不是none,返回给定的Option<T>
        let x = Some(2);
        let y: Option<&str> = Some("2");
        let z = x.and(y);
        println!("{:?}",z);

        // and_then 有计算操作;如果是none则返回none,不是none返回 调用闭包的值
        let z =  x.and_then(|x| Some(x * x));
        println!("{:?}",z)
    }
}