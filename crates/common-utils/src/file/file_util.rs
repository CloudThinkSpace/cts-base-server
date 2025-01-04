use std::path::Path;

/// 获取文件扩展名
///
/// @param file_name 文件名
pub fn get_ext(file_name: &str) -> Option<String> {
    let path = Path::new(file_name);
    let extension = path.extension();
    extension.map(|data| data.to_string_lossy().to_string())
}
