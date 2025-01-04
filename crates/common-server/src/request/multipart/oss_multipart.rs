use axum::extract::Multipart;
use crate::request::CtsFile;
use crate::request::multipart::CtsOssParse;
use anyhow::{Error, Result};
use axum::extract::multipart::Field;
use common_storage::oss::Oss;
use common_storage::Storage;
use crate::config::oss_config::get_config;
use bytes::Buf;

impl CtsOssParse for Multipart {
    async fn parse(&mut self) -> Result<Vec<CtsFile>> {
        let mut oss_vec = Vec::new();
        while let Some(field) = self.next_field().await? {
            // 处理文件
            if let Some(_filename) = field.file_name() {
                let oss_path = stream_to_oss(field).await?;
                oss_vec.push(oss_path);
            }
        }
        Ok(oss_vec)
    }
}

async fn stream_to_oss(stream: Field<'_>) -> Result<CtsFile> {
    let file_name = match &stream.file_name() {
        Some(data) => data.to_string(),
        None => "".to_string(),
    };
    // 数据
    let body = stream.bytes().await?;
    let config = get_config().await;
    let oss_config = config.oss.ok_or(Error::msg("没有oss配置信息"))?;
    let oss = Oss::new(oss_config.key_id, oss_config.key_secret, oss_config.endpoint, oss_config.bucket);
    let oss_path = format!("/{}", oss_config.path);
    let (filename, path) = oss.write(&file_name, &oss_path, body.chunk()).await?;
    Ok(CtsFile { filename, path })
}