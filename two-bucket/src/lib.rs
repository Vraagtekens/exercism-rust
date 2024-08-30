#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}


/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {

    let (mut first, first_cap, mut second, second_cap) = match start_bucket {
        Bucket::One => (capacity_1, capacity_1, 0, capacity_2),
        Bucket::Two => (capacity_2, capacity_2, 0, capacity_1)
    };
    let mut moves: u8 = 1;
    let init_first = first;
    let init_second = second;
    
    while first != goal && second != goal {
        if second_cap == goal {
            second = second_cap;
        } else if first == 0 {
            first = first_cap;
        } else if second == second_cap {
            second = 0;
        } else if first > second_cap - second {
            first -= second_cap - second;
            second = second_cap;
        } else {
            second += first;
            first = 0;
        }
        moves += 1;
        if first == init_first && second == init_second {
            return None;
        }
    };
    let (goal_bucket, other_bucket) = match *start_bucket {
        Bucket::One if first == goal => (Bucket::One, second),
        Bucket::One                  => (Bucket::Two, first),
        Bucket::Two if first == goal => (Bucket::Two, second),
        Bucket::Two                  => (Bucket::One, first)
    };
    Some(BucketStats {
        moves,
        goal_bucket,
        other_bucket
    })
    
}
