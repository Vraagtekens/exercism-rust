static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(usize),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: usize, b: usize) -> Result<String, AffineCipherError> {
    if check_coprime(a) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let x: String = plaintext
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .enumerate()
        .map(|(i, c)| {
            let mut letter = String::new();
            if i != 0 && i % 5 == 0 {
                letter.push(' ');
            }

            // check
            if c.is_numeric() {
                letter.push(c);
                return letter;
            }

            let v = char_to_index(c).unwrap() as usize;

            letter.push(index_to_char((a * v + b) % 26).unwrap());
            letter
        })
        .collect();

    Ok(x)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if check_coprime(a as usize) {
        return Err(AffineCipherError::NotCoprime(a as usize));
    }

    let x: String = ciphertext
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| {
            if c.is_numeric() {
                return c;
            }

            let v = char_to_index(c).unwrap() as i32;
            let mmh = find_modular_inverse(a, 1);

            index_to_char((mmh * (v - b)).rem_euclid(26).try_into().unwrap()).unwrap()
        })
        .collect();

    Ok(x)
}

fn check_coprime(num: usize) -> bool {
    26 % num == 0 || num % 2 == 0
}

fn index_to_char(index: usize) -> Option<char> {
    ALPHABET.chars().nth(index)
}

fn char_to_index(c: char) -> Option<usize> {
    ALPHABET.find(c)
}

fn find_modular_inverse(num: i32, mut index: i32) -> i32 {
    if num * index % 26 == 1 {
        return index;
    }

    index += 1;

    find_modular_inverse(num, index)
}
