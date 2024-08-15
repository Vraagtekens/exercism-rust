use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&'a str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut anagrams = HashSet::new();

    let sorted_word: Vec<char> = {
        let mut chars: Vec<char> = word.to_lowercase().chars().collect();
        chars.sort_unstable();
        chars
    };

    for &possible in possible_anagrams {
        if possible.to_lowercase() == word.to_lowercase() {
            continue;
        }

        let sorted_possible: Vec<char> = {
            let mut chars: Vec<char> = possible.to_lowercase().chars().collect();
            chars.sort_unstable();
            chars
        };

        if sorted_word == sorted_possible {
            anagrams.insert(possible);
        }
    }

    anagrams
}
