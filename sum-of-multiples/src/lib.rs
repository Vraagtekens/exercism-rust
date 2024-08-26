use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // (1..limit)
    //     .filter(|x| divs.iter().any(|d| *d != 0 && x % d == 0))
    //     .sum()

    let mut list = HashSet::new();
    for factor in factors{
        if *factor == 0 {
            continue;
        };

        let mut x: u32 = *factor;
        while x < limit{
            list.insert(x);
            x += factor;
        }
    }

    let list: Vec<u32> = list.into_iter().collect();
    list.iter().sum()
}
