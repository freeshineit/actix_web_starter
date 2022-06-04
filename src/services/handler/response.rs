use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct Response<T> {
    /// 0 失败  1 成功
    code: 0 | 1,
    msg: String
    data: T
}