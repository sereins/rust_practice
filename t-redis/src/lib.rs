pub mod cmd;
pub use cmd::Command;

pub mod frame;
pub use frame::Frame;

pub mod db;
use db::Db;
pub mod server;

mod parse;
use parse::{Parse, ParseError};

pub mod shutdown;

use shutdown::Shutdown;

pub mod connection;
use connection::Connection;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
