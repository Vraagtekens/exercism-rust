use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {

    // input.chunks(worker_count);
    // let x = input.chunks(worker_count);

    let chunks = input.chunks(worker_count);
    let hm = Arc::new(Mutex::new(HashMap::new()));
    // let mut handles = vec![];

    // for string in input {
    //     let hm = Arc::clone(&hm);

    //     // for ch in string.to_lowercase().chars() {
    //     //     if ch.is_alphabetic(){
    //     //         *hm.entry(ch).or_insert(0) += 1;
    //     //     }
    //     // }

    //     // Spawn a thread for each chunk
    //     let handle = thread::spawn(move || {
    //         let mut local_map = HashMap::new();
    //         for ch in string.chars() {
    //             *local_map.entry(ch).or_insert(0) += 1;
    //         }

    //         // Merge local result into the shared map
    //         let mut shared_map = hm.lock().unwrap();
    //         for (key, value) in local_map {
    //             *shared_map.entry(key).or_insert(0) += value;
    //         }
    //     });

    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    Arc::try_unwrap(hm).unwrap().into_inner().unwrap()
}



// test bench_large_parallel   ... bench:     168,288.34 ns/iter (+/- 34,323.79)
// test bench_large_sequential ... bench:     235,100.13 ns/iter (+/- 93,648.95)
// test bench_small_parallel   ... bench:       6,382.69 ns/iter (+/- 1,386.41)
// test bench_small_sequential ... bench:       8,203.78 ns/iter (+/- 4,986.20)
// test bench_tiny_parallel    ... bench:          84.56 ns/iter (+/- 13.59)
// test bench_tiny_sequential  ... bench:          40.20 ns/iter (+/- 6.71)
pub fn _frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {

    let mut hm = HashMap::new();

    for string in input {
        for ch in string.to_lowercase().chars() {
            if ch.is_alphabetic(){
                *hm.entry(ch).or_insert(0) += 1;
            }
        }
    }

    hm
}
