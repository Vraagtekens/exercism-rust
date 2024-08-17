pub fn is_armstrong_number(num: u32) -> bool {

    let binding = num.to_string();
    let chars = binding.chars();
    let mut count: u32 = 0;

    for char in chars {
        let numeric_value: u32 = char.to_digit(10).unwrap_or(0);
        count += numeric_value.pow(binding.len() as u32);
    }

    num == count
}
