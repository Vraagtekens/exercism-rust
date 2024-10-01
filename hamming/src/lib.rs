/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {

    if s1.len() != s2.len() {
        return None
    }

    let x: Vec<usize> = s1
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if Some(c) != s2.chars().nth(i) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    Some(x.len())

    // if s1.len() != s2.len(){
    //     None
    // }else{
    //     Some(
    //         s1.chars().zip(s2.chars())
    //         .filter( |(c1,c2)| c1 != c2)
    //         .count()
    //     )
    // }
}
