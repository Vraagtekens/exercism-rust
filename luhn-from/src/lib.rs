pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let result: Option<u32> = self
            .0
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

        println!("{:?}", result);

        match result {
            Some(sum) => sum % 10 == 0 && sum != 0,
            None => false,
        }
    }
}

// impl<T: ToString> From<T> for Luhn { // cleaner
impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        // todo!("From the given input '{input}' create a new Luhn struct.");
        Luhn(input.to_string())
    }
}
