use std::{collections::HashMap, sync::mpsc, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let (tx, rx) = mpsc::channel::<HashMap<char, usize>>();

    let chunk_size = {
        let x = input.len() / worker_count;
        if x == 0 {
            1
        } else {
            x
        }
    };

    for chunk in input.chunks(chunk_size) {
        let chunk = chunk.iter().map(|c| c.to_string()).collect::<Vec<String>>();
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let mut list: HashMap<char, usize> = HashMap::new();
            chunk.iter().for_each(|sentence| {
                sentence
                    .to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .for_each(|c| {
                        *list.entry(c).or_insert(0) += 1;
                    });
            });

            tx_clone.send(list).unwrap();
        });
    }

    drop(tx);

    let mut list: HashMap<char, usize> = HashMap::new();
    for partial_map in rx.iter() {
        partial_map.iter().for_each(|(char, count)| {
            list.entry(*char)
                .and_modify(|s| *s += count)
                .or_insert(*count);
        });
    }

    list
}
