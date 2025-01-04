use std::fmt::Display;

/// 补齐字符函数
///
/// @param input 输入字符串
///
/// @param pad 待补齐字符
///
/// @param width 字符串长度
pub fn padding_char<S: Display>(input: S, pad: char, width: usize) -> String {
    // 转换成字符串
    let input_string = input.to_string();
    // 判断字符串是否大于width
    if input_string.len() >= width {
        return input_string;
    }
    // 待补齐的个数
    let padding_needs = width - input_string.len();
    let mut padding_string = String::with_capacity(width);
    // 添加补齐字符
    for _ in 0..padding_needs {
        padding_string.push(pad)
    }
    // 添加输入字符串
    padding_string.push_str(&input_string);

    padding_string
}


#[cfg(test)]
mod test {
    use crate::string::padding_char;

    #[test]
    pub fn txt() {
        let aa = padding_char("aaa", '0', 10);
        println!("{aa}");
    }
}