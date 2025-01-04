use crate::Storage;
use anyhow::Result;
use chrono::{Datelike, Local};
use common_utils::file::file_util::get_ext;
use std::fs;
use std::fs::create_dir_all;
use uuid::Uuid;
pub struct LocalFile;

impl Storage for LocalFile {
    async fn read(&self, path: &str) -> Result<Vec<u8>> {
        let data = fs::read(path)?;
        Ok(data)
    }
    async fn write(&self, file_name: &str, path: &str, data: &[u8]) -> Result<(String, String)> {
        // 穿件唯一名
        let uuid = Uuid::new_v4().to_string();
        // 扩展名
        let exp = get_ext(file_name);
        let file_path = create_file_dir(path);
        // 创建嵌套文件夹
        create_dir_all(&file_path)?;
        // 文件路径
        let path = match exp {
            None => format!("{file_path}/{uuid}"),
            Some(data) => format!("{file_path}/{uuid}.{data}"),
        };
        // 写入数据
        fs::write(&path, data)?;
        // 返回文件名和路径
        Ok((file_name.to_string(), path))
    }
}

/// 创建时间目录
///
/// @param path 根目录
///
/// @return path 返回实际路径，日期目录：2025-01-01
fn create_file_dir(path: &str) -> String {
    let date = Local::now();
    let time = format!(
        "{}/{}-{}-{}",
        path,
        date.year(),
        pad_char(date.month()),
        pad_char(date.day())
    );
    time
}

fn pad_char(input: u32) -> String {
    format!("{:0>2}", input)
}

#[cfg(test)]
mod test {
    use crate::local::create_file_dir;

    #[test]
    fn aa() {
        let aa = create_file_dir("public");
        println!("{aa}")
    }
}