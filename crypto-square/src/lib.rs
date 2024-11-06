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
            r += 1;
        } else {
            c += 1;
        }
    }

    // println!("{:?}", r);
    // println!("{:?}", c);

    if x.is_empty() {
        return x;
    }

    let e = x.clone();
    let e: Vec<String> = e
        .chars()
        .collect::<Vec<_>>()
        .chunks(r)
        .map(|chunk| chunk.iter().collect())
        .collect();

    let mut meow: Vec<String> = vec![];
    for i in 0..c {
        println!("{:?}", i);
        let mut string = String::new();
        for word in e.clone() {
            let c = word.chars().nth(i).unwrap_or(' ');
            string.push(c);
        }

        meow.push(string);
    }

    meow.join(" ")
}
