use anyhow::Error;
use axum::extract::Multipart;
use axum::extract::multipart::Field;
use bytes::Buf;
use common_storage::local::LocalFile;
use common_storage::Storage;
use crate::config::file_config::get_config;
use crate::request::CtsFile;
use crate::request::multipart::CtsLocalParse;
use anyhow::Result;

impl CtsLocalParse for Multipart {
    async fn parse(&mut self) -> Result<Vec<CtsFile>> {
        let mut oss_vec = Vec::new();
        while let Some(field) = self.next_field().await? {
            // 处理文件
            if let Some(_filename) = field.file_name() {
                let oss_path = stream_to_file(field).await?;
                oss_vec.push(oss_path);
            }
        }
        Ok(oss_vec)
    }
}

async fn stream_to_file(stream: Field<'_>) -> Result<CtsFile> {
    let file_name = match &stream.file_name() {
        Some(data) => data.to_string(),
        None => "".to_string(),
    };
    // 数据
    let body = stream.bytes().await?;
    let config = get_config().await;
    let file_config = config.upload.ok_or(Error::msg("没有oss配置信息"))?;
    let local_file = LocalFile;
    let file_path = format!("/{}", file_config.path);
    let (filename, path) = local_file.write(&file_name, &file_path, body.chunk()).await?;
    Ok(CtsFile { filename, path })
}