pub fn factors(mut n: u64) -> Vec<u64> {

    let mut expected = Vec::new();
    let mut count = 2;

    loop {
        if n % count == 0 {
            expected.push(count);
            n /= count;
            count = 2;
        } else {
            count += 1;
        }

        if count > n {
            break;
        }
    }
    
    expected
}
