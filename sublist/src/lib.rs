#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    // Check for equality
    if first_list == second_list {
        return Comparison::Equal;
    }

    // Check if first_list is a sublist of second_list
    if first_list.is_empty() || second_list.windows(first_list.len()).any(|window| window == first_list) {
        return Comparison::Sublist;
    }

    // Check if second_list is a sublist of first_list
    if second_list.is_empty() || first_list.windows(second_list.len()).any(|window| window == second_list) {
        return Comparison::Superlist;
    }

    // If none of the above, return Unequal
    Comparison::Unequal
}
