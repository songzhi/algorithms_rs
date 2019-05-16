//!There are people sitting in a circular fashion,
//!print every third member while removing them,
//!the next counter starts immediately after the member is removed.
//!Print till all the members are exhausted.
//!
//!For example:
//!Input: consider 123456789 members sitting in a circular fashion,
//!Output: 369485271

pub fn josephus<T: Clone>(array: &[T], skip: usize) -> Vec<T> {
    let mut array = array.to_vec();
    let mut result = Vec::with_capacity(array.len());
    let mut idx = 0;
    while array.len() > 0 {
        idx = (skip - 1 + idx) % (array.len());
        result.push(array.remove(idx));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_josephus() {
        let array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = vec![3, 6, 9, 4, 8, 5, 2, 7, 1];
        assert_eq!(result, josephus(&array, 3));
    }
}