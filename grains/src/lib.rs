pub fn square(s: u32) -> u64 {
    if s == 1 {
        return s as u64
    }

    (2 as u64).pow(s - 1)
}

pub fn total() -> u128 {


    square(64) as u128 * 2 - 1 
}
