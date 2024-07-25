use std::result;

pub enum Errors {}

/// 重写系统 result 类型，以满足自定义错误类型的需要
pub type Result<T> = result::Result<T, Errors>;
