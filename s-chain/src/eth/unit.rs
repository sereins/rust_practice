// u256 单位的处理
// parse_units 将给定的字符串转换为以太饭单位，第一个参数是我们常见的单位，第二个参数是幂或者一个表示幂的字符串
// format_units 将u256 转换成我们常见的单位。第一个参数是u256，第二个参数是幂或者一个表示幂的字符串

#[cfg(test)]
mod tests {
    use alloy::primitives::utils::{format_units, parse_ether, parse_units};
    use alloy::primitives::U256;

    #[test]
    fn test_parse_units() {
        // 将str 类型转换为 u256
        let pu = parse_units("1.0", 18).unwrap();
        let num: U256 = pu.into();

        let pu = parse_units("0.1", 18).unwrap();
        let num2: U256 = pu.into();

        let pu = parse_ether("1").unwrap();
        let num3: U256 = pu.into();

        println!("num1:{}", num);
        println!("num2:{}", num2);
        println!("num3:{}", num3);
    }
    #[test]
    fn test_format_units() {
        let pu = parse_units("10.0", 18).unwrap();
        let num: U256 = pu.into();

        let format = format_units(num, 18).unwrap();
        println!("format:{}", format);
    }
}
