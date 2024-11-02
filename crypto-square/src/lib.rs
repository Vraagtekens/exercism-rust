pub fn encrypt(input: &str) -> String {
    let x: String = input
        .to_lowercase()
        .chars()
        .filter(|x| x.is_ascii_alphanumeric())
        .collect::<String>();

    // println!("{:?}", x);
    let mut r = 0;
    let mut c = 0;
    while !(r * c >= x.len() && c >= r && c - r <= 1) {
        if r == c {
            c += 1;
        } else {
            r += 1;
        }
    }

    println!("{:?}", r);
    println!("{:?}", c);

    x
}
