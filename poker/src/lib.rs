use std::cmp::Ordering;
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

    let mut list: Vec<(&str, PokerHand)> = hands
        .iter()
        .map(|&hand_str| {
            let hand_type = get_poker_hand_type(hand_str);
            let card_ranks = extract_sorted_ranks(hand_str);
            let poker_hand = PokerHand {
                hand_type,
                card_ranks,
            };
            (hand_str, poker_hand)
        })
        .collect();

    list.sort_by(|a, b| b.1.cmp(&a.1)); // sort strongest to weakest

    let best_score = list[0].1.clone();
    let result: Vec<&str> = list
        .into_iter()
        .take_while(|(_, hand)| *hand == best_score)
        .map(|(hand_str, _)| hand_str)
        .collect();

    result
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct PokerHand {
    hand_type: PokerHandType, // compared first
    card_ranks: Vec<u8>,      // descending order
}

fn get_card_value(card: &str) -> u32 {
    match card.chars().next() {
        Some(c) => match c {
            'A' => 14,
            'K' => 13,
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
    let is_ace_low_straight = values == vec![2, 3, 4, 5, 14];
    let is_straight = is_straight || is_ace_low_straight;

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

impl Ord for PokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then_with(|| self.card_ranks.cmp(&other.card_ranks))
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn extract_sorted_ranks(hand: &str) -> Vec<u8> {
    // Count frequencies of each rank
    let mut freq_map: HashMap<u8, usize> = HashMap::new();
    for card in hand.split_whitespace() {
        let val = get_card_value(card) as u8;
        *freq_map.entry(val).or_insert(0) += 1;
    }

    // Collect into a Vec<(rank, count)>
    let mut counts: Vec<(u8, usize)> = freq_map.into_iter().collect();

    // Sort primarily by count descending, then rank descending
    counts.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));

    // Flatten into vector: repeat the rank count times
    let mut sorted_ranks = Vec::new();
    for (rank, count) in counts {
        for _ in 0..count {
            sorted_ranks.push(rank);
        }
    }

    // Handle ace-low straight case (optional, if relevant for card ranks)
    if sorted_ranks == vec![14, 5, 4, 3, 2] {
        return vec![5, 4, 3, 2, 1];
    }

    sorted_ranks
}
