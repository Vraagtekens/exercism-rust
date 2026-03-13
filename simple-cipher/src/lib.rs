pub fn encode(key: &str, s: &str) -> Option<String> {

    s.chars().enumerate().map(|(u, c)| {

        let u = 26 % u;
        let k = key.chars().nth(u).unwrap() as u32;
        let c = char::from_u32(c as u32).unwrap() as u32;
        let bruh = ((c - 'a' as u32) + (k - 'a' as u32)) % 26 + 'a' as u32;


        char::from_u32(bruh)
    }).collect::<Option<String>>()
}

pub fn decode(key: &str, s: &str) -> Option<String> {

    s.chars().enumerate().map(|(u, c)| {
        let k = key.chars().nth(u).unwrap() as u32;
        let c = char::from_u32(c as u32).unwrap() as u32;
        let bruh = ((c - 'a' as u32) - (k - 'a' as u32)) % 26 + 'a' as u32;


        char::from_u32(bruh)
    }).collect::<Option<String>>()
}

pub fn encode_random(s: &str) -> (String, String) {
    todo!("Generate random key with only a-z chars and encode {s}. Return tuple (key, encoded s)")
}


