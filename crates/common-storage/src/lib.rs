use std::future::Future;
use anyhow::Result;

pub mod oss;
pub mod local;

/// 存储接口
pub trait Storage {

    /// 读取数据
    ///
    /// @param path 读取的路径
    fn read(&self, path: &str) -> impl Future<Output=Result<Vec<u8>>> + Send;

    /// 写入数据
    ///
    /// @param file_name 文件名
    ///
    /// @param path 文件路径
    ///
    /// @param data 写入的数据
    fn write(&self, file_name: &str, path: &str, data: &[u8]) -> impl Future<Output=Result<(String, String)>> + Send;
}


