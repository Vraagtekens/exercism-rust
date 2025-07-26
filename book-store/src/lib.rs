use std::collections::HashMap;

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut list: HashMap<u32, Vec<u32>> = HashMap::new();

    for num in books.iter() {
        let mut index: u32 = 0;

        loop {
            // Use entry API
            let entry = list.entry(index).or_default();

            if !entry.contains(num) {
                entry.push(*num);
                break;
            }

            index += 1;
        }
    }

    let mut total: u32 = 0;
    for (_, books) in list {
        let books_total: u32 = books.len().try_into().unwrap();
        let procent: u32 = match books_total {
            5 => 75,
            4 => 80,
            3 => 90,
            2 => 95,
            _ => 100,
        };

        total += 800 * books_total / 100 * procent;
    }

    total
}
