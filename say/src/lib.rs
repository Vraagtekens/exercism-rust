
pub fn encode(n: u64) -> String {

    if n == 0 {
        return "zero".to_string();
    }

    let mut result: Vec<String> = Vec::new();
    let mut remainder = n;

    let quintillions = remainder / 1_000_000_000_000_000_000;
    remainder %= 1_000_000_000_000_000_000;
    let quadrillions = remainder / 1_000_000_000_000_000;
    remainder %= 1_000_000_000_000_000;
    let trillions = remainder / 1_000_000_000_000;
    remainder %= 1_000_000_000_000;
    let billions = remainder / 1_000_000_000;
    remainder %= 1_000_000_000;
    let millions = remainder / 1_000_000;
    remainder %= 1_000_000;
    let thousands = remainder / 1_000;
    remainder %= 1_000;


    if quintillions > 0 {
        result.push(format!("{} quintillion", big_number(quintillions)));
    }
    if quadrillions > 0 {
        result.push(format!("{} quadrillion", big_number(quadrillions)));
    }
    if trillions > 0 {
        result.push(format!("{} trillion", big_number(trillions)));
    }
    if billions > 0 {
        result.push(format!("{} billion", big_number(billions)));
    }
    if millions > 0 {
        result.push(format!("{} million", big_number(millions)));
    }
    if thousands > 0 {
        result.push(format!("{} thousand", big_number(thousands)));
    }
    if remainder > 0 {
        result.push(big_number(remainder));
    }
    
    result.join(" ")
}

// 0..999
fn big_number(n: u64) -> String{
    let mut x: Vec<String> = Vec::new();
    let y = n / 100;
    
    if n > 99 {
        x.push(format!("{} hundred", small_number(y)));
    }

    let y = n % 100;
    if n % 100 != 0 {
        match y {
            91..=99 => x.push(format!("ninety-{}", small_number(y % 90))),
            90 => x.push("ninety".to_string()),
            81..=89 => x.push(format!("eighty-{}", small_number(y % 80))),
            80 => x.push("eighty".to_string()),
            71..=79 => x.push(format!("seventy-{}", small_number(y % 70))),
            70 => x.push("seventy".to_string()),
            61..=69 => x.push(format!("sixty-{}", small_number(y % 60))),
            60 => x.push("sixty".to_string()),
            51..=59 => x.push(format!("fifty-{}", small_number(y % 50))),
            50 => x.push("fifty".to_string()),
            41..=49 => x.push(format!("forty-{}", small_number(y % 40))),
            40 => x.push("forty".to_string()),
            31..=39 => x.push(format!("thirty-{}", small_number(y % 30))),
            30 => x.push("thirty".to_string()),
            21..=29 => x.push(format!("twenty-{}", small_number(y % 20))),
            20 => x.push("twenty".to_string()),
            19 => x.push("nineteen".to_string()),
            18 => x.push("eighteen".to_string()),
            17 => x.push("seventeen".to_string()),
            16 => x.push("sixteen".to_string()),
            15 => x.push("fifteen".to_string()),
            14 => x.push("fourteen".to_string()),
            13 => x.push("thirtteen".to_string()),
            12 => x.push("twelve".to_string()),
            11 => x.push("eleven".to_string()),
            10 => x.push("ten".to_string()),
            _ => x.push(small_number(y))
        }
    }

    x.join(" ")
}

// 1..9
fn small_number(n: u64) -> String{

    let x = match n {
        9 => "nine",
        8 => "eight",
        7 => "seven",
        6 => "six",
        5 => "five",
        4 => "four",
        3 => "three",
        2 => "two",
        1 => "one",
        _ => "one"
    };

    x.to_string()
}
