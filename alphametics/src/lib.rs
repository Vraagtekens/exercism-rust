use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut letters: Vec<char> = input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<Vec<char>>();
    letters.sort();
    letters.dedup();
    
    let equation: Vec<&str> = input.split("==").collect();
    let left_side: Vec<&str> = equation[0].split('+').map(|s| s.trim()).collect();
    let right_side = equation[1].trim();

    let mut digits: Vec<u8> = (0..10).collect();
    permute(&mut digits, 0, &letters, &left_side, right_side)
}

fn permute(digits: &mut Vec<u8>, start: usize, letters: &[char], left_side: &[&str], right_side: &str) -> Option<HashMap<char, u8>> {
    if start == letters.len() {
        let mapping: HashMap<char, u8> = letters.iter().cloned().zip(digits[..letters.len()].iter().cloned()).collect();
        
        // Check if any word starts with a letter mapped to 0
        if mapping.iter().any(|(&c, &d)| d == 0 && (left_side.iter().chain(std::iter::once(&right_side)).any(|&s| s.chars().next() == Some(c)))) {
            return None;
        }

        let left_sum: u64 = left_side
            .iter()
            .map(|s| s.chars().fold(0, |acc, c| acc * 10 + mapping[&c] as u64))
            .sum();

        let right_value: u64 = right_side
            .chars()
            .fold(0, |acc, c| acc * 10 + mapping[&c] as u64);

        if left_sum == right_value {
            return Some(mapping);
        }
    } else {
        for i in start..digits.len() {
            digits.swap(i, start);
            if let Some(solution) = permute(digits, start + 1, letters, left_side, right_side) {
                return Some(solution);
            }
            digits.swap(i, start);
        }
    }
    None
}