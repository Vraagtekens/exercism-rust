#[derive(Debug, Clone)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    //no items, no value
    let Some(next_item) = items.first() else {
        return 0;
    };

    let value_without_item = maximum_value(max_weight, &items[1..]);
    
    //too heavy, get the max value without this item
    if next_item.weight > max_weight {
        return value_without_item;
    }
    //return the highest value if we include or exclude the item
    let value_with_item = next_item.value + maximum_value(max_weight - next_item.weight, &items[1..]);
    
    value_without_item.max(value_with_item)
}

pub fn maximum_value2(max_weight: u32, items: &[Item]) -> u32 {
    let all_subsets = generate_subsets(items, 0);
    let mut total_arr: Vec<Item> = all_subsets
        .iter()
        .map(|subset| {
            subset.iter().fold(
                Item {
                    weight: 0,
                    value: 0,
                },
                |acc, item| Item {
                    weight: acc.weight + item.weight,
                    value: acc.value + item.value,
                },
            )
        })
        .collect();

    total_arr.sort_by(|a, b| b.value.cmp(&a.value));

    // total_arr.
    // println!("{:?}", total_arr);
    let result = total_arr
        .iter()
        .find(|item| item.weight <= max_weight)
        .map(|item| item.value);

    result.unwrap_or(0)
}

// fn generate_subsets(arr: &[Item], current: Vec<Item>, index: usize) {
//     // Base case: if we've considered all elements, print the current subset
//     if index == arr.len() {
//         println!("{:?}", current);
//         return;
//     }
//     // Recursive case 1: Exclude the current element and move to the next
//     generate_subsets(arr, current.clone(), index + 1);

//     // Recursive case 2: Include the current element and move to the next
//     let mut new_subset = current.clone();
//     new_subset.push(arr[index].clone());
//     generate_subsets(arr, new_subset, index + 1);
// }

fn generate_subsets(arr: &[Item], index: usize) -> Vec<Vec<Item>> {
    // Base case: return an empty subset when we reach the end of the array
    if index == arr.len() {
        return vec![vec![]]; // Return a vector containing an empty subset
    }

    // Recursive case: Get all subsets that do not include the current element
    let mut subsets_without_current = generate_subsets(arr, index + 1);

    // Now, create new subsets by adding the current element to each subset
    let mut subsets_with_current = Vec::new();
    for subset in &subsets_without_current {
        let mut new_subset = subset.clone(); // Clone the subset
        new_subset.push(arr[index].clone()); // Add the current element
        subsets_with_current.push(new_subset);
    }

    // Combine the subsets that include and don't include the current element
    subsets_without_current.extend(subsets_with_current);
    subsets_without_current
}
