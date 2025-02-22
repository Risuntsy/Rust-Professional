/*
    Find Intersection of Two Arrays
    Given two arrays, find the intersection of the arrays and return the elements of the intersection (without duplicates).
    The result should not contain any duplicate elements.

    You need to implement the function `intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the elements that are in both arrays.

    Hint: You can solve this problem using sorting, hash sets, or the two-pointer technique.
*/

use std::fmt::{self, Display, Formatter};

pub fn next_index<T: PartialEq>(nums: &Vec<T>, index: &mut usize) {
    while *index < nums.len() - 1 && nums[*index] == nums[*index + 1] {
        *index += 1;
    }
    *index += 1;
}

pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    nums1.sort_unstable();
    nums2.sort_unstable();

    let (mut left, mut index_1, mut index_2) = (0, 0, 0);

    while index_1 < nums1.len() && index_2 < nums2.len() {
        match &nums1[index_1].cmp(&nums2[index_2]) {
            std::cmp::Ordering::Less => {
                next_index(&mut nums1, &mut index_1);
            }
            std::cmp::Ordering::Equal => {
                nums1[left] = nums1[index_1];
                left += 1;
                next_index(&mut nums1, &mut index_1);
                next_index(&mut nums2, &mut index_2);
            }
            std::cmp::Ordering::Greater => {
                next_index(&mut nums2, &mut index_2);
            }
        }
    }

    nums1.get(0..left).expect("Invalid index").to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_intersection_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![4, 9]);
    }

    #[test]
    fn test_intersection_3() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4, 5, 6];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_intersection_4() {
        let nums1 = vec![1, 1, 1];
        let nums2 = vec![1, 1, 1];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_intersection_5() {
        let nums1 = vec![10, 20, 30];
        let nums2 = vec![30, 40, 50];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![30]);
    }
}
