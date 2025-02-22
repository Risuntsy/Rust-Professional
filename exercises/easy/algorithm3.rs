/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T>(array: &mut [T])
where
    T: PartialOrd + ToOwned<Owned = T>,
{
    if array.len() <= 1 {
        return;
    }

    quick_sort::<T>(array, 0, (array.len() -1) as isize);
}

fn quick_sort<T>(arr: &mut [T], start: isize, end: isize)
where
    T: PartialOrd + ToOwned<Owned = T>,
{
    if start >= end || arr.len() <= 1 {
        return;
    }

    let (pivot, mut left, mut right) = ((&arr[(start + end) as usize / 2]).to_owned(), start, end);

    while left < right {
        while arr[left as usize] < pivot {
            left += 1;
        }
        
        while arr[right as usize] > pivot {
            right -= 1;
        }

        if left <= right {
            arr.swap(left as usize, right as usize);
            left+=1;
            right-=1;
        }
    }

    quick_sort(arr, start, right);
    quick_sort(arr, left, end);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}