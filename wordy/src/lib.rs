pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is") && !command.ends_with("?") {
        return None;
    }

    let binding = command
        .replace("What is", "")
        .replace("?", "")
        .replace("by", "");
    let words: Vec<&str> = binding.split_whitespace().collect();

    let mut words = words.iter();
    let first = words.next()?;
    let mut sum = first.parse::<i32>().ok()?;

    while let Some(op) = words.next() {
        let next_number = words.next()?;
        match *op {
            "plus" => sum += next_number.parse::<i32>().ok()?,
            "minus" => sum -= next_number.parse::<i32>().ok()?,
            "multiplied" => sum *= next_number.parse::<i32>().ok()?,
            "divided" => sum /= next_number.parse::<i32>().ok()?,
            "raised" => {
                let the = words.next()?;
                let exponent_str = words.next()?;
                let power = words.next()?;

                if *the != "the" || *power != "power" {
                    return None;
                }

                let exp = exponent_str
                    .trim_end_matches(|c: char| c.is_alphabetic())
                    .parse::<u32>()
                    .ok()?;

                sum = sum.pow(exp);
            }

            _ => return None,
        }
    }

    Some(sum)
}
