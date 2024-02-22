/// 1. and  and_then  于之 相反的为 or or_else
/// 2. map:对里面的值在进行一次处理后 map_err:可以进行错误类型转化。
/// 3. map_or: 对Some进行计算，否则返回默认 map_or_else
/// 4. map_or_orr :如果ok返回执行结果，错误返回错误的闭包.
/// 5. ok_or,ok_or_else 转换成 Result类型
/// 6. take 获取Option里面的值
/// 6. unwrap---unwrap_err unwrap_or--unwrap_or_default---unwrap_or_else
#[cfg(test)]
mod tests{
    #[test]
    fn t_and(){
        let mut x:Option<u32> = None;
        let c =x.unwrap_or(1);
        println!("{:?}",c)
    }
}