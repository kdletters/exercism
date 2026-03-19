use std::cmp::min;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.is_empty() && second_list.is_empty() {
        return Comparison::Equal;
    } else if first_list.is_empty() {
        return Comparison::Sublist;
    } else if second_list.is_empty() {
        return Comparison::Superlist;
    }

    let window_size = min(first_list.len(), second_list.len());
    if first_list.len() == window_size {
        if second_list
            .windows(window_size)
            .any(|window| window == first_list)
        {
            if first_list.len() == second_list.len() {
                return Comparison::Equal;
            }
            return Comparison::Sublist;
        }
    } else {
        if first_list
            .windows(window_size)
            .any(|window| window == second_list)
        {
            return Comparison::Superlist;
        }
    }

    Comparison::Unequal
}
