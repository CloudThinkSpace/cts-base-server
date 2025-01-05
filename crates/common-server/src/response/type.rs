use axum::http::{HeaderMap, HeaderName, HeaderValue};

pub enum FileType {
    Image,
    Video,
    Other,
}

pub fn parse_type(path: String, file_type: FileType) -> HeaderMap {
    // 查找是否有点符号
    let index = path.find('.').unwrap_or(usize::MAX);
    //文件扩展名
    let mut ext_name = "png";
    if index != usize::MAX {
        ext_name = &path[index + 1..];
    }
    let content_type = match file_type {
        FileType::Image => {
            format!("image/{}", ext_name)
        }
        FileType::Video => "video/*".to_string(),
        FileType::Other => "application/octet-stream".to_string(),
    };
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_str(&content_type).unwrap(),
    );
    headers
}