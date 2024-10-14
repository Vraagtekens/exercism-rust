use core::num;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // todo!("Out of {hands:?}, which hand wins?")

    let mut arr: Vec<&str> = vec![];
    if hands.len() == 1 {
        let x = hands.first().unwrap();
        return vec![x];
    }

    let number_hands: Vec<Vec<u32>> = hands
        .iter()
        .map(|x| {
            let bruh: Vec<&str> = x.split_ascii_whitespace().collect();
            let mut score: Vec<u32> = vec![];
            for card in bruh {
                score.push(get_card_value(card));
            }
            score.sort_by(|a, b| {
                // let bruh = b.cmp(a);
                // println!("{:?}{}", a, b);
                // println!("{:?}", bruh);
                b.cmp(a)
            });
            score
        })
        .collect();

    println!("{:?}", number_hands);
    number_hands.iter().enumerate().for_each(|(i, hand)| {
        let x = hand.iter().nth(i).unwrap();
    });

    arr
}

fn get_card_value(card: &str) -> u32 {
    match card.chars().next() {
        Some(c) => match c {
            'A' => 14,
            'H' => 13,
            'Q' => 12,
            'J' => 11,
            '1' => 10,
            c => c.to_digit(10).unwrap(),
        },
        None => 0,
    }
}
