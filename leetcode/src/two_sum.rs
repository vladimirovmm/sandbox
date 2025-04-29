//! Найт индексы двух чисел в массиве, которые дают в сумме заданное число.
//! Например, если nums = [2, 7, 11, 15] и target = 9, то ответ [0, 1].
//! Нельзя использовать элементы массива дваждажды.
//! Предпологается что всегда есть одно решение.
//!
//!

#![cfg(test)]

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (p_a, v_a) in nums.iter().enumerate() {
        for (p_b, v_b) in nums.iter().enumerate().skip(p_a + 1) {
            if v_a + v_b == target {
                return vec![p_a as i32, p_b as i32];
            }
        }
    }

    Default::default()
}

fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (p_a, v_a) in nums.iter().enumerate() {
        let b = target - *v_a;
        if let Some(p_b) = nums.iter().skip(p_a + 1).position(|v| v == &b) {
            return vec![p_a as i32, (p_b + p_a + 1) as i32];
        }
    }

    Default::default()
}

fn two_sum_3(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let last_index = nums.len() - 1;
    let mut index_a = 0;
    loop {
        let mut index_b = index_a + 1;
        let need_value = target - nums[index_a];

        loop {
            if nums[index_b] == need_value {
                return vec![index_a as i32, index_b as i32];
            }

            index_b += 1;
            if index_b > last_index {
                break;
            }
        }

        index_a += 1;
        if index_a > last_index {
            break;
        }
    }

    Default::default()
}

#[test]
fn test_two_sum() {
    for f in [two_sum, two_sum_2, two_sum_3] {
        assert_eq!(f(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(f(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(f(vec![3, 3], 6), vec![0, 1]);
    }
}
