const VOWELS: [char; 5] = ['a', 'u', 'e', 'i', 'o'];

pub fn translate(input: &str) -> String {
    if input.is_empty() {
        return "".to_string();
    }

    let list: Vec<&str> = input.split_whitespace().collect();

    let x: Vec<String> = list
        .iter()
        .map(|x| {
            let word = x.to_lowercase();
            let first_letter = word.chars().next().unwrap();
            let second_letter = word.chars().nth(1).unwrap_or('\0');

            match first_letter {
                c if VOWELS.contains(&c) => format!("{}ay", x),
                'x' if second_letter == 'r' => format!("{}ay", x),
                'y' if second_letter == 't' => format!("{}ay", x),
                _ => switch_a_roo(word),
            }
        })
        .collect();

    x.join(" ")
}

fn switch_a_roo(word: String) -> String {
    let vowel_index: Option<usize> = word.chars().enumerate().find_map(|(index, c)| {
        if c == 'q' {
            if let Some('u') = word.chars().nth(index + 1) {
                return Some(index + 2);
            }
        }

        if c == 'y' && index > 0 {
            return Some(index);
        }

        if VOWELS.contains(&c) {
            return Some(index);
        }

        None
    });

    match vowel_index {
        Some(index) => {
            let (first_part, rest) = word.split_at(index);

            format!("{}{}ay", rest, first_part)
        }
        None => word,
    }
}
