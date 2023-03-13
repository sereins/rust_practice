use std::borrow::{Borrow, BorrowMut};
use mysql::Pool;

/// 获取数据库连接池
pub fn get_pool() -> Pool {
    let uri = "mysql://test:a123456@localhost:3306/test";
    Pool::new(uri).unwrap()
}