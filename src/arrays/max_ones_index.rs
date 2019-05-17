//!Find the index of 0 to be replaced with 1 to get
//!longest continuous sequence
//!of 1s in a binary array.
//!Returns index of 0 to be
//!replaced with 1 to get longest
//!continuous sequence of 1s.
//!If there is no 0 in array, then
//!it returns -1.
//!
//!e.g.
//!let input array = [1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1]
//!If we replace 0 at index 3 with 1, we get the longest continuous
//!sequence of 1s in the array.
//!So the function return => 3

pub fn max_ones_index(array: &[u8]) -> usize {
    let mut max_count = 0;
    let mut max_index = 0;
    let mut prev_zero = -1;
    let mut prev_prev_zero = -1;
    for curr in 0..array.len() {
        // If current element is 0, then calculate the difference
        // between curr and prev_prev_zero
        if array[curr] == 0 {
            if curr as isize - prev_prev_zero > max_count {
                max_count = curr as isize - prev_prev_zero;
                max_index = prev_zero;
            }
            prev_prev_zero = prev_zero;
            prev_zero = curr as isize;
        }
    }
    if array.len() as isize - prev_prev_zero > max_count {
        max_index = prev_zero;
    }
    max_index as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_ones_index() {
        let array = [1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1];
        assert_eq!(3, max_ones_index(&array));
    }
}