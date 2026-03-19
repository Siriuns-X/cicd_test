/// 将字符串每个单词的首字母大写
/// "hello world" -> "Hello World"
pub fn capitalize_words(s: &str) -> String {
    // TODO: 补全实现
    s.split_ascii_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        assert_eq!(capitalize_words("hello world"), "Hello World");
    }
    #[test]
    fn test_single() {
        assert_eq!(capitalize_words("rust"), "Rust");
    }
    #[test]
    fn test_empty() {
        assert_eq!(capitalize_words(""), "");
    }
}
