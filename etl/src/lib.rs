use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut arr: BTreeMap<char, i32> = BTreeMap::new();

    for (a, b) in h {
        for c in b {
            arr.insert(c.to_ascii_lowercase(), *a);
        }
    }

    arr
}
