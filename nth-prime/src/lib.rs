pub fn nth(n: u32) -> u32 {

    if n < 4 {
        return [2, 3, 5, 7][n as usize];
    }
    
    let mut primes = vec![2, 3, 5, 7];
    let mut candidate = 11;
    
    while primes.len() <= n as usize {
        if is_prime(candidate) {
            primes.push(candidate);
        }
        candidate += 2;  // We can skip even numbers
    }
    
    primes[n as usize]
}


fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u32;
    for &prime in &[2, 3, 5, 7] {
        if n % prime == 0 {
            return n == prime;
        }
    }
    for i in (11..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}