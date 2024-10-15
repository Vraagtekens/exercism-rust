#[derive(Debug, Clone)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    // todo!("calculate the maximum value achievable with the given {items:?} and {max_weight}");

    // let items: Vec<Item> = items.sort_by(|a, b| a.value.cmp(&b.value)).collect();
    // items.sort_by(|a, b| a.value.cmp(&b.value));
    // for item in items {}

    let total = items.iter().fold(
        Item {
            weight: 0,
            value: 0,
        },
        |acc, item| Item {
            weight: acc.weight + item.weight,
            value: acc.value + item.value,
        },
    );

    let mut mmh: Vec<Item> = vec![];
    for item in items {
        mmh.push(item.clone());
    }

    let x = 0;
    x
}
