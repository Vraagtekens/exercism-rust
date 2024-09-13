
/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
        
    let vec: Vec<char> = sentence
        .to_lowercase()
        .chars()
        .filter(|c| {
            c.is_alphabetic()
        })
        .collect();

    (0..26)
        .map(|i| (b'a' + i) as char) 
        .all(|x| vec.contains(&x))

    // sentence
    //     .to_lowercase()
    //     .chars()
    //     .filter(|c| c.is_ascii_alphabetic())
    //     .collect::<HashSet<char>>()
    //     .len()
    //     == 26
    
    // let s = sentence.to_lowercase();
    // ('a'..='z').all(|b| s.contains(b))
}
