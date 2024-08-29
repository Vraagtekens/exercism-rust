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

    let mut moves: u8 = 0;
    let first = match start_bucket {
        Bucket::One => capacity_1,
        Bucket::Two => capacity_2,
    };
    let second = match start_bucket {
        Bucket::One => capacity_2,
        Bucket::Two => capacity_1,
    };
    let mut bucket_1 = 0;
    let mut bucket_2 = 0;
    while goal != bucket_1 {
        moves += 1;

        if bucket_2 == second{
            bucket_2 = 0
        } else if bucket_1 == 0 {
            bucket_1 = first
        } else {
            let transfer_amount = bucket_1.min(second - bucket_2);
            bucket_1 -= transfer_amount;
            bucket_2 += transfer_amount;
        }

        println!("{}", bucket_1);
        println!("{}\n", bucket_2);
        
    }

    Some(BucketStats{
        moves: moves,
        goal_bucket: start_bucket.clone(),
        other_bucket: bucket_2
    })
}
