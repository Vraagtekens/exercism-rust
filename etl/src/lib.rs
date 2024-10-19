use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut arr: BTreeMap<char, i32> = BTreeMap::new();

    for x in h {
        for c in x.1 {
            let lower: char = c.to_lowercase().next().unwrap();
            arr.insert(lower, *x.0);
        }
    }

    arr
}
