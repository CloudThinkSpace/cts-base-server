use aliyun_oss_rust_sdk::{oss::OSS, request::RequestBuilder};
use anyhow::{Ok};
use chrono::{Datelike, Local, Timelike};
use uuid::Uuid;
use common_utils::file::file_util::get_ext;
use crate::Storage;
use anyhow::Result;

pub struct Oss {
    aliyun_oss: OSS,
}

impl Oss {
    pub fn new<S: Into<String>>(key_id: S, key_secret: S, endpoint: S, bucket: S) -> Self {
        Self {
            aliyun_oss: OSS::new(key_id, key_secret, endpoint, bucket)
        }
    }
}

impl Storage for Oss {
    async fn read(&self, path: &str) -> Result<Vec<u8>> {
        let oss = &self.aliyun_oss;
        let build = RequestBuilder::new();
        let body = oss.get_object(path, build).await?;
        Ok(body)
    }

    async fn write(&self, file_name: &str, path: &str, data: &[u8]) -> Result<(String, String)> {
        let oss = &self.aliyun_oss;
        let build = RequestBuilder::new();
        let oss_path = create_oss_dir(path);
        let uuid = Uuid::new_v4().to_string();
        // 扩展名
        let exp = get_ext(file_name);
        // 文件路径
        let path = match exp {
            None => format!("{oss_path}/{uuid}"),
            Some(data) => format!("{oss_path}/{uuid}.{data}"),
        };
        // 上传文件
        oss.pub_object_from_buffer(&path, data, build).await?;
        Ok((file_name.to_string(), path))
    }
}

/// 创建时间目录
///
/// @param path 根目录
///
/// @return path 返回实际路径
fn create_oss_dir(path: &str) -> String {
    let date = Local::now();
    let time = format!(
        "{}/{}/{}/{}/{}/{}",
        path,
        date.year(),
        date.month(),
        date.day(),
        date.hour(),
        date.minute()
    );
    time
}
