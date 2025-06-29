static PLAIN: &str = "abcdefghijklmnopqrstuvwxyz";
static CIPHER: &str = "zyxwvutsrqponmlkjihgfedcba";

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let result = map_characters(plain, PLAIN, CIPHER).unwrap();

    // Separate character every 5 chars
    let mut cipher = String::new();
    result.chars().enumerate().for_each(|c| {
        if c.0 % 5 == 0 && !cipher.is_empty() {
            cipher.push(' ');
        }

        cipher.push(c.1);
    });

    cipher
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    map_characters(cipher, CIPHER, PLAIN).unwrap()
}

fn map_characters(sentence: &str, current_map: &str, map: &str) -> Option<String> {
    let mut plain = String::new();

    for c in sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
    {
        if c.is_numeric() {
            plain.push(c);
        }

        if c.is_ascii_alphabetic() {
            let i = current_map.find(c)?;
            let ch = map.chars().nth(i)?;
            plain.push(ch);
        }
    }

    Some(plain)
}
