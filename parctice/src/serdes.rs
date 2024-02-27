use serde::{Serialize, Deserialize};

/// 1.重命名
/// 2.默认值 默认值，默认函数
/// 3.自定义序列化规则
/// 4.平
#[derive(Debug,Serialize, Deserialize)]
enum Cycle {
    Year,
    Day(u32)
}

#[test]
fn test_cycle() {
    println!("hello world");
}