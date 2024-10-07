/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn new(value: u64) -> Option<Palindrome> {
        let string = value.to_string();
        let reversed_string: String = string.chars().rev().collect();
        if string == reversed_string {
            return Some(Palindrome(value));
        }

        None
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut products_vec: Vec<Palindrome> = vec![];

    for x in min..=max {
        for y in x..=max {
            let product = Palindrome::new(x * y);

            match product {
                Some(x) => products_vec.push(x),
                None => {}
            }
        }
    }

    println!("{:?}", products_vec);

    products_vec.sort();
    let first = products_vec.first();
    let last = products_vec.last();

    match (first, last) {
        (Some(first), Some(last)) => Some((*first, *last)),
        _ => None,
    }
}
