/*
    Merge Intervals
    Given an array of intervals where each interval is represented by a pair of integers [start, end],
    merge all overlapping intervals and return a list of non-overlapping intervals.

    The intervals are inclusive, meaning the interval [start, end] includes both start and end points.

    You need to implement the function `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
    The function should return a vector containing all the merged intervals.

    Hint: You can start by sorting the intervals by their starting point and then merge them one by one.
*/

use std::fmt::{self, Display, Formatter};

pub fn merge_intervals(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.len() <= 1 {
        return intervals;
    }

    intervals.sort_unstable_by_key(|a| a[0]);

    let len = intervals.len();
    let (mut left, mut right) = (0, 1);

    while right < len {
        if &intervals[left][1] >= &intervals[right][0] {
            intervals[left][1] = intervals[right][1].max(intervals[left][1]);
            right += 1;
        } else {
            intervals[left + 1] = intervals[right].to_owned();
            left += 1;
            right += 1;
        }
    }

    intervals
        .get(0..=left)
        .expect("Failed to get merged intervals")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn test_merge_intervals_2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 5]]);
    }

    #[test]
    fn test_merge_intervals_3() {
        let intervals = vec![vec![1, 4], vec![0, 4]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![0, 4]]);
    }

    #[test]
    fn test_merge_intervals_4() {
        let intervals = vec![vec![1, 10], vec![2, 6], vec![8, 10]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 10]]);
    }

    #[test]
    fn test_merge_intervals_5() {
        let intervals = vec![vec![1, 2], vec![3, 5], vec![4, 7], vec![8, 10]];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![vec![1, 2], vec![3, 7], vec![8, 10]]);
    }
}
