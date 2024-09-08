
pub fn series(digits: &str, len: usize) -> Vec<String> {
    // let mut vec = Vec::new();
    // if len == 0 || len > digits.len() {
    //     return vec;
    // }

    // // Generate substrings of specified length
    // for i in 0..=(digits.len() - len) {
    //     vec.push(digits[i..i + len].to_string());
    // }

    // vec


    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|s| s.iter().collect::<String>())
        .collect()
}
