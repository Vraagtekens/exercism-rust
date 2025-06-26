use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let input_map = input
        .to_ascii_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|f| (f, 0))
        .collect::<HashMap<char, u8>>();

    let mut x = input.split("==");

    let mut leading: Vec<char> = vec![];
    let first: Vec<&str> = x.next().unwrap().split_ascii_whitespace().collect();
    let last = x.next().unwrap().trim();

    if last.len() > 1 {
        let c = last.chars().next().unwrap();
        leading.push(c);
    }

    first.iter().for_each(|f| {
        if f.len() > 1 {
            let c = f.chars().next().unwrap();
            leading.push(c);
        }
    });

    let mut end_map: HashMap<char, u8> = HashMap::new();
    for perm in (0..10)
        .collect::<std::vec::Vec<u8>>()
        .iter()
        .permutations(input_map.len())
        .unique()
    {
        if input_map
            .iter()
            .zip(perm.iter())
            .any(|(ch, digit)| leading.contains(ch.0) && **digit == 0)
        {
            continue; // skip this entire permutation
        }

        // safe to insert now
        for (ch, digit) in input_map.iter().zip(perm.iter()) {
            end_map.insert(*ch.0, **digit);
        }

        let left: Vec<u64> = first
            .iter()
            .map(|str| str_to_u64(str, &end_map))
            .collect::<Vec<_>>();
        let left_sum: u64 = left.iter().sum();

        let right: u64 = str_to_u64(last, &end_map);

        if left_sum == right {
            return Some(end_map);
        }
    }

    None
}

fn str_to_u64(str: &str, map: &HashMap<char, u8>) -> u64 {
    str.chars()
        .map(|f| map.get(&f).unwrap_or(&0))
        .fold(0, |acc, digit| acc * 10 + (*digit as u64))
}
