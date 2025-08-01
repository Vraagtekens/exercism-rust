use std::collections::HashMap;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 1 {
        let x = hands.first().unwrap();
        return vec![x];
    }

    let mut arr: Vec<&str> = vec![];

    let mut list: Vec<PokerHand> = vec![];
    for hand in hands {
        let mut card_total: u32 = 0;
        for card in hand.split_ascii_whitespace() {
            card_total += get_card_value(card);
        }

        let poker_hand_type = get_poker_hand_type(hand);
        let poker_hand = PokerHand::new(poker_hand_type, card_total);

        list.push(poker_hand);
    }

    list.sort();
    list.reverse();

    arr
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PokerHandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PokerHand {
    hand_type: PokerHandType, // compared first
    total_value: u32,         // compared second
}

impl PokerHand {
    fn new(hand_type: PokerHandType, total_value: u32) -> Self {
        Self {
            hand_type,
            total_value,
        }
    }
}

fn get_card_value(card: &str) -> u32 {
    match card.chars().next() {
        Some(c) => match c {
            'A' => 14,
            'H' => 13,
            'Q' => 12,
            'J' => 11,
            '1' => 10, // 10
            c => c.to_digit(10).unwrap(),
        },
        None => 0,
    }
}

fn get_poker_hand_type(hand: &str) -> PokerHandType {
    // Step 1: Parse the hand into Vec<(rank, suit)>
    let cards: Vec<&str> = hand.split_whitespace().collect();

    // Step 2: Count rank frequencies
    let mut frequencies: HashMap<u32, u32> = HashMap::new();
    for card in cards {
        // frequencies.insert(k, v) get_card_value(card);
        let value = get_card_value(card);
        *frequencies.entry(value).or_insert(0) += 1;
    }

    // Step 3: Check for flush
    let cards: Vec<&str> = hand.split_whitespace().collect();
    let first_suit = cards[0].chars().last().unwrap();
    let is_flush = cards
        .iter()
        .all(|card| card.chars().last().unwrap() == first_suit);

    // Step 4: Check for straight
    let mut values: Vec<u32> = frequencies.keys().cloned().collect();
    values.sort();

    let is_straight = values.len() == 5 && values.windows(2).all(|w| w[1] == w[0] + 1);

    // Step 5: Match against rules, in order from highest to lowest
    let mut counts: Vec<u32> = frequencies.values().cloned().collect();
    counts.sort();
    match (is_flush, is_straight, &counts[..]) {
        (true, true, _) => PokerHandType::StraightFlush,
        (_, _, [1, 5]) => PokerHandType::FiveOfAKind,
        (_, _, [1, 4]) => PokerHandType::FourOfAKind,
        (_, _, [2, 3]) => PokerHandType::FullHouse,
        (true, _, _) => PokerHandType::Flush,
        (_, true, _) => PokerHandType::Straight,
        (_, _, [1, 1, 3]) => PokerHandType::ThreeOfAKind,
        (_, _, [1, 2, 2]) => PokerHandType::TwoPair,
        (_, _, [1, 1, 1, 2]) => PokerHandType::OnePair,
        _ => PokerHandType::HighCard,
    }
}
