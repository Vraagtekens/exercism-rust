pub fn egg_count(display_value: u32) -> usize {

    let binary_string = format!("{:b}", display_value);
    binary_string
        .chars()
        .filter(|&c| c == '1')
        .count()
}
