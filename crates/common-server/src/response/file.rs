use anyhow::{Error, Result};
use common_storage::local::LocalFile;
use common_storage::oss::Oss;
use common_storage::Storage;
use crate::config::oss_config;

pub async  fn get_file(path:&str, storage_type: StorageType) -> Result<Vec<u8>>{

    match storage_type {
        StorageType::Local => {
            let file = LocalFile;
            file.read(path).await
        }
        StorageType::Oss => {
            let config = oss_config::get_config().await;
            let oss_config = config.oss.ok_or(Error::msg("没有oss配置信息"))?;
            let oss = Oss::new(oss_config.key_id, oss_config.key_secret, oss_config.endpoint, oss_config.bucket);
            oss.read(path).await
        }
    }

}

pub enum StorageType {
    Local,
    Oss
}
