use num_bigint::BigUint;
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    // todo!("Pick a private key greater than 1 and less than {p}")
    let mut rng = rand::thread_rng();
    let random_number: u64 = rng.gen_range(2..p);

    random_number
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // Convert values to BigUint
    let p = BigUint::from(p);
    let g = BigUint::from(g);
    let a = BigUint::from(a);

    let result = g.modpow(&a, &p);

    result.to_u64_digits()[0]
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // todo!("Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}")
    // s = Bᵃ mod p

    // Convert values to BigUint
    let p = BigUint::from(p);
    let b_pub = BigUint::from(b_pub);
    let private_key = BigUint::from(a);

    // Calculate B^a mod p
    let result = b_pub.modpow(&private_key, &p);

    // Convert the result back to u64 if it fits
    result.to_u64_digits()[0]
}
