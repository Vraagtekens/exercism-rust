/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false;
    }

    let check_chars = isbn.chars().all(|c| c.is_numeric() || c == 'X' || c == '-');
    if !check_chars {
        return false;
    }

    let mut list: Vec<u32> = vec![];
    let all_chars: Vec<char> = isbn
        .chars()
        .filter(|c| c.is_numeric() || c == &'X')
        .collect();

    for (index, char) in all_chars.iter().rev().enumerate() {
        let index: u32 = (index + 1).try_into().unwrap();
        let result = char.to_digit(10);

        match result {
            Some(number) => list.push(number * index),
            None => {
                if char == &'X' && index == 1 {
                    list.push(10 * index);
                } else {
                    return false;
                }
            }
        };
    }

    if list.len() != 10 {
        return false;
    }

    list.iter().sum::<u32>() % 11 == 0
}
