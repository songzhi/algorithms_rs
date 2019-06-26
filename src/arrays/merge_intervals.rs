//!In mathematics, a (real) interval is a set of real
//! numbers with the property that any number that lies
//! between two numbers in the set is also included in the set.


///A set of real numbers with methods to determine if other
///numbers are included in the set.
///Includes related methods to merge and print interval sets.
pub struct Interval {
    start: isize,
    end: isize,
}