pub fn recite(mut start_bottles: u32, take_down: u32) -> String {
    let mut i: u32 = 0;
    let mut settence: Vec<String> = vec![];

    while take_down > i {
        let first_sentence = format!("{} hanging on the wall,", number_to_string(start_bottles));

        start_bottles -= 1;

        let last_sentence = if start_bottles == 0 {
            String::from("There'll be no green bottles hanging on the wall.")
        } else {
            format!(
                "There'll be {} hanging on the wall.",
                number_to_string(start_bottles).to_lowercase(),
            )
        };

        let part = format!(
            "{}\n{}\n{}\n{}",
            first_sentence,
            first_sentence,
            "And if one green bottle should accidentally fall,",
            last_sentence
        );

        settence.push(part);

        i += 1;
    }

    settence.join("\n\n")
}

fn number_to_string(number: u32) -> String {
    let word = match number {
        1 => "One green bottle",
        2 => "Two green bottles",
        3 => "Three green bottles",
        4 => "Four green bottles",
        5 => "Five green bottles",
        6 => "Six green bottles",
        7 => "Seven green bottles",
        8 => "Eight green bottles",
        9 => "Nine green bottles",
        10 => "Ten green bottles",
        _ => "Zero",
    };

    word.to_string()
}
