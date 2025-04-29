//! Высчитывает медиану двух массивов.
//!
//! Медина двух массивов находится в середине объединенного массива.
//! Если кол-во элементов в массиве нечетное, то медиана это среднее значение двух соседних элементов.
//!

#![cfg(test)]

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums: Vec<&i32> = nums1.iter().chain(&nums2).collect();
    nums.sort();

    let len = nums.len();
    let mid = len / 2;
    if len % 2 == 0 {
        (nums[mid] + nums[mid - 1]) as f64 / 2.0
    } else {
        *nums[mid] as f64
    }
}

#[test]
fn test_find_median_sorted_arrays() {
    assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    assert_eq!(find_median_sorted_arrays(vec![0, 0], vec![0, 0]), 0.0);
}
