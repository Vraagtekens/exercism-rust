#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    let arr = string_digits.chars().collect::<Vec<char>>();

    let mut array: Vec<u64> = vec![];

    let mut span_index = span;
    for (i, _) in string_digits.chars().enumerate() {
        let chars: Vec<char> = arr[i..span_index].into();

        let mut num: u32 = 1;
        for c in chars {
            let digit = c.to_digit(10);

            match digit {
                Some(c) => num *= c,
                _ => return Err(Error::InvalidDigit(c)),
            }
        }
        array.push(num.into());

        if span_index >= arr.len() {
            break;
        } else {
            span_index += 1;
        }
    }

    array.sort();

    match array.last() {
        Some(x) => Ok(*x),
        _ => Ok(0),
    }
}
