
pub fn encode(source: &str) -> String {

    // AABBBCCCC -> 2A3B4C

    let mut num = 1;
    let mut result = String::new();
    for (index, c) in source.char_indices(){

        if let Some(x) = source.chars().nth(index + 1){
            if x == c {
                num += 1;
            } else {
                let format = if num == 1 {format!("{c}")} else {format!("{num}{c}")};
                result.push_str(&format);
                num = 1;
            }
        } else {
            let format = if num == 1 {format!("{c}")} else {format!("{num}{c}")};
            result.push_str(&format);
            num = 1;
        }   
    }

    result
}

pub fn decode(source: &str) -> String {

    let mut num: String = String::new();
    let mut result = String::new();
    for c in source.chars(){

        if c.is_numeric() {
            num.push_str(&c.to_string());
        } else {
            if !num.is_empty() {
                match num.parse::<u32>() {
                    Ok(number) => {
                        (0..number).for_each(|_| result.push_str(&c.to_string()));
                    }, 
                    Err(_) => println!("Failed to parse the string as u32"),
                }
            } else {
                result.push_str(&c.to_string())
            }

            num = String::new();
        }
    }

    result
}
