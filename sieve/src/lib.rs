pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut list: Vec<(u64, bool)> = (2..=upper_bound).map(|n| (n, false)).collect();

    for num in 2..=upper_bound {
        list = list
            .iter()
            .map(|n| {
                if n.0 % num == 0 && n.0 != num {
                    return (n.0, true);
                }

                *n
            })
            .collect::<Vec<(u64, bool)>>();
    }

    list.iter()
        .filter(|(_, b)| !*b)
        .map(|(n, _)| *n)
        .collect::<Vec<u64>>()
}
