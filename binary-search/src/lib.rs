use std::cmp::Ordering;

pub fn find<T, A>(array: A, key: T) -> Option<usize> 
where
    T: Ord,
    A: AsRef<[T]>,
{
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        match array[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,
            Ordering::Greater => {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
        }
    }

    None

    // match array.binary_search(&key) {
    //     Ok(t) => Some(t),
    //     Err(_) => None
    // }
}