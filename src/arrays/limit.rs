//!Sometimes you need to limit array result to use. Such as you only need the
//! value over 10 or, you need value under than 100. By use this algorithms, you
//! can limit your array to specific value
//!
//!If array, Min, Max value was given, it returns array that contains values of
//! given array which was larger than Min, and lower than Max. You need to give
//! 'unlimit' to use only Min or Max.
//!
//!ex) limit([1,2,3,4,5], None, 3) = [1,2,3]
//!
//!Complexity = O(n)


pub fn limit<T: Ord + Clone>(array: &[T], min_limit: Option<T>, max_limit: Option<T>) -> Vec<T> {
    match (min_limit, max_limit) {
        (None, Some(max_limit)) => array.iter().filter(|&x| *x <= max_limit).map(|x| x.clone()).collect(),
        (Some(min_limit), None) => array.iter().filter(|&x| *x >= min_limit).map(|x| x.clone()).collect(),
        (Some(min_limit), Some(max_limit)) => array.iter().filter(|&x| *x <= max_limit && *x >= min_limit).map(|x| x.clone()).collect(),
        _ => array.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit() {
        let array = [1, 2, 3, 4, 5];
        let result = vec![1, 2, 3];
        assert_eq!(result, limit(&array, None, Some(3)));
    }
}