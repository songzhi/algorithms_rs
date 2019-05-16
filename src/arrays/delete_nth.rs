//!Given a list lst and a number N, create a new list
//!that contains each number of the list at most N times without reordering.
//
//!For example if N = 2, and the input is [1,2,3,1,2,1,2,3], you take [1,2,3,1,2],
//!drop the next [1,2] since this would lead to 1 and 2 being in the result 3 times, and then take 3,
//!which leads to [1,2,3,1,2,3]


use std::collections::HashMap;
use std::hash::Hash;

/// Time complexity O(n^2)
pub fn delete_nth_naive<T: Eq + Clone>(array: &[T], n: usize) -> Vec<T> {
    let mut result = Vec::new();
    for num in array {
        if result.iter().filter(|&n| *n == *num).count() < n {
            result.push(num.clone());
        }
    }
    result
}

/// Time Complexity O(n), using hash tables.
pub fn delete_nth<T: Eq + Clone + Hash>(array: &[T], n: usize) -> Vec<T> {
    let mut result = Vec::new();
    let mut counts = HashMap::<T, usize>::with_capacity(array.len());

    for num in array {
        if counts.get(num).map(|num| *num < n).unwrap_or(true) {
            result.push(num.clone());
            counts.insert(num.clone(), counts.get(num).map(|n| *n + 1).unwrap_or(1));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_nth_naive() {
        let array = [1, 2, 3, 1, 2, 1, 2, 3];
        assert_eq!(vec![1, 2, 3, 1, 2, 3], delete_nth_naive(&array, 2));
    }

    #[test]
    fn test_delete_nth() {
        let array = [1, 2, 3, 1, 2, 1, 2, 3];
        assert_eq!(vec![1, 2, 3, 1, 2, 3], delete_nth(&array, 2));
    }
}