//!Find missing ranges between low and high in the given array.
//!Ex) [3, 5] lo=1 hi=10 => answer: [(1, 2), (4, 4), (6, 10)]

pub fn missing_ranges(array: &[isize], low: isize, high: isize) -> Vec<(isize, isize)> {
    let mut result = vec![];
    let mut start = low;
    for &n in array {
        if n == start {
            start += 1;
        } else if n > start {
            result.push((start, n - 1));
            start = n + 1;
        }
    }
    if start <= high {
        result.push((start, high));
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = vec![(1, 2), (4, 4), (6, 10)];
        assert_eq!(result, missing_ranges(&[3, 5], 1, 10));
    }
}