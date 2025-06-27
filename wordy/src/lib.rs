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
    let first = words.next()?; // first should be a number
    let mut sum = first.parse::<i32>().ok()?; // parse string to i32

    println!("{:?}", sum);
    println!("{:?}", words);

    while let Some(op) = words.next() {
        let next_number = words.next()?; // safely get next item
        match *op {
            "plus" => sum += next_number.parse::<i32>().ok()?,
            "minus" => sum -= next_number.parse::<i32>().ok()?,
            "multiplied" => sum *= next_number.parse::<i32>().ok()?,
            "divided" => sum /= next_number.parse::<i32>().ok()?,
            // add more: "multiplied", "divided"
            _ => return None,
        }
    }

    Some(sum)
}
