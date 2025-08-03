pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let s = self.to_string();
        let result: Option<u32> = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .rev()
            .enumerate()
            .map(|c| {
                let digit = c.1.to_digit(10)?;
                let i = c.0 + 1;
                if i % 2 == 0 {
                    let num = digit * 2;
                    if num > 9 {
                        return Some(num - 9);
                    }
                    return Some(num);
                }

                Some(digit)
            })
            .sum();

        match result {
            Some(sum) => sum % 10 == 0 && sum != 0,
            None => false,
        }
    }
}

// impl<T: ToString> From<T> for Luhn {

//     fn from(input: T) -> Self {
//         // todo!("From the given input '{input}' create a new Luhn struct.");
//         Luhn(input.to_string())
//     }
// }
