pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0{
        return None
    }

    let mut count = 0;
    loop {
        if n == 1{
            break Some(count);
        }

        if n % 2 == 0{
            n /= 2;
        } else {
            n = n * 3 + 1;
        }

        count += 1;
    }
}
