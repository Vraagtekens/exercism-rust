pub fn abbreviate(phrase: &str) -> String {
    // Split on any non-letter character except apostrophes
    let words: Vec<&str> = phrase
        .split(|c: char| !c.is_alphabetic() && c != '\'')
        .filter(|s| !s.is_empty())
        .collect();

    let mut result = String::new();

    // println!("{:?}", words);
    let mut prev_char_is_lowercase = false;
    for word in words {
        // Remove any remaining special characters and get first letter
        if word.is_empty() {
            continue;
        }

        // Remove special characters
        let cleaned = word.replace(['_', '\''], "");

        // Handle each character to detect camelCase
        for (i, c) in cleaned.chars().enumerate() {
            // Add first character of each word
            if i == 0 {
                result.push(c.to_ascii_uppercase());
                prev_char_is_lowercase = c.is_lowercase();
                continue;
            }

            // If we transition from lowercase to uppercase, it's a new word
            if prev_char_is_lowercase && c.is_uppercase() {
                result.push(c);
            }

            prev_char_is_lowercase = c.is_lowercase();
        }
    }

    result
}
