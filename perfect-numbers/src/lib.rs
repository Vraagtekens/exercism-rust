// use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {

    if num == 0 {
        return None
    }

    let sum: u64 = (1..num).filter(|x| num % x == 0).sum();

    match sum {
        x if x == num => Some(Classification::Perfect),  // Perfect number
        x if x > num => Some(Classification::Abundant),  // Abundant number
        x if x < num => Some(Classification::Deficient), // Deficient number
        _ => None, // This case is actually unreachable, but added for safety
    }


    // if num == 0 {
    //     None
    // } else {
    //     match (1 .. num).filter(|&f| num % f == 0).sum::<u64>().cmp(&num) {
    //         Ordering::Less => Some(Classification::Deficient),
    //         Ordering::Equal => Some(Classification::Perfect),
    //         Ordering::Greater => Some(Classification::Abundant)
    //     }
    // }
}
