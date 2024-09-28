use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {

    let mut x: HashMap<String, u32> = HashMap::new();

    let words = words.split(|c: char| 
        c.is_whitespace() || 
        c == ',' ||
        c == '\n');

    for word in words {
        let cleaned_word = word
            .trim_matches(|c: char| !c.is_alphabetic() && !c.is_numeric())
            .to_lowercase().to_string();

        if !cleaned_word.is_empty() {
            *x.entry(cleaned_word).or_insert(0) += 1;
        }
    }

    println!("{:?}", x);

    x
}
