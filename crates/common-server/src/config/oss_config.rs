use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use crate::config::{read_config_content};

static GLOBAL_CONFIG: OnceCell<Config> = OnceCell::const_new();

pub async fn get_config() -> Config {
    GLOBAL_CONFIG
        .get_or_init(|| async { Config::init_config() })
        .await
        .clone()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub oss: Option<OssConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OssConfig {
    pub key_id: String,
    pub key_secret: String,
    pub endpoint: String,
    pub bucket: String,
    pub path: String,
}

impl Config {
    /// 默认读取项目目录下的配置文件config.toml
    pub fn init_config() -> Config {
        // 配置文件默认路径
        let file_path = "config.yml";
        let content = read_config_content(file_path);
        serde_yaml::from_str(&content).expect("Parse config file error")
    }
}