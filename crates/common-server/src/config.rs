use std::fs::File;
use std::io::Read;
pub mod oss_config;
pub mod file_config;


/// 读取配置文件内容
///
/// @param path 配置文件路径
pub fn read_config_content(path: &str) -> String {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", path, e),
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e),
    };
    str_val
}

