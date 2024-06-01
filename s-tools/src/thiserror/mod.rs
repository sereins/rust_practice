use std::io;
use thiserror::Error;

/// summary
/// #[error] 宏:本质作用是为字段实现display 特征。
/// #[from io::Error] 宏:本质上是是自动实现From特征，从一个类型转换为另一个类型。
///
/// anyhow:anyhow::Result可以兼容任何实现了std::error::Error类型的。方便与其他库做交互。
/// anyhow:可以将实现了std::error::Error类型的错误转化为anyhow::Error,实现了From特征。

pub fn first_char(s: &String) -> char {
    if s.len() == 0 {
        '-'
    } else {
        s.chars().next().unwrap_or('-')
    }
}

#[derive(Debug)]
pub struct Limits {
    lo: i16,
    hi: i16,
}
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum TheError {
    #[error("invalid rdo_lookahead_frames {0} (expected < {})", i32::MAX)]
    InvalidLookahead(u32),
    #[error("first letter must be lowercase but was {:?}", first_char(.0))]
    WrongCase(String),
    #[error("invalid index {idx}, expected at least {} and at most {}", .limits.lo, .limits.hi)]
    OutOfBounds { idx: usize, limits: Limits },
}

#[derive(Error, Debug)]
pub enum ErrorFrom {
    #[error("io error from")]
    Transfer(#[from] io::Error),
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::anyhow;
    use std::error::Error;

    #[test]
    fn test_error() {
        let error1 = TheError::InvalidLookahead(30);
        println!("error1 = {}", error1);

        let error2 = TheError::WrongCase("this is goods".to_string());
        println!("error2 = {}", error2);

        let limit = Limits { lo: 0, hi: 10 };
        let error3 = TheError::OutOfBounds {
            idx: 12,
            limits: limit,
        };
        println!("error3 = {}", error3);
    }

    #[test]
    fn test_from() {
        let error1 = ErrorFrom::from(io::Error::new(io::ErrorKind::AddrNotAvailable, "not use"));
        println!("error1 = {}", error1);
        println!("source = {:?}", error1.source());

        let io_error = io::Error::new(io::ErrorKind::AlreadyExists, "exists");
        let error2: ErrorFrom = io_error.into();
        println!("error2 = {}", error2);
    }

    #[test]
    fn test_anyhow() {
        let error = anyhow!("this is anyhow error");
        println!("anyhow error = {}", error);
    }
}
