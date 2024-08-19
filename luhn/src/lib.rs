/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {

    // code.chars()
    //     .rev()
    //     .filter(|c| !c.is_whitespace())
    //     .try_fold((0, 0), |(sum, count), val| {
    //         val.to_digit(10)
    //             .map(|num| if count % 2 == 1 { num * 2 } else { num })
    //             .map(|num| if num > 9 { num - 9 } else { num })
    //             .map(|num| (num + sum, count + 1))
    //     }).map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)

    if code.len() <= 1 {
        return false
    }

    let numbers: Vec<u32> = code.chars()
        .filter(|&x| x != ' ') 
        .rev() 
        .enumerate()
        .filter_map(|(i, x)| {
            x.to_digit(10).map(|mut digit| {
                if i % 2 == 1 {
                    digit *= 2;
                    if digit > 9 {
                        digit -= 9;
                    }
                }
                digit
            })
        })
        .collect(); 

    if numbers.len() != code.chars().filter(|&x| x != ' ').count() {
        return false;
    }

    if numbers.len() <= 1 {
        return false;
    }

    let mut sum: u32 = 0;
    for num in numbers {

        sum += num
    }

    sum % 10 == 0
}
