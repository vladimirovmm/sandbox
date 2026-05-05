//! Тестирование различных типов сортировок.
//! Сравнить со стандартной сортировкой.
//!

use std::fmt::Debug;

/// Сортировка пузырьком (Bubble Sort)
/// Простой, но неэффективный алгоритм сортировки с временной сложностью O(n²).
/// Работает путем многократного прохода по массиву и сравнения соседних элементов.
/// Если элементы находятся в неправильном порядке, они меняются местами.
///
/// # Примеры
///
/// ```
/// use exp_sort::bubble_sort;
///
/// let mut arr = [2, 3, 1];
/// bubble_sort(&mut arr);
/// assert_eq!(arr, [1, 2 ,3]);
/// ```
///
/// ```
/// use exp_sort::bubble_sort;
///
/// let mut arr = [5, 2, 8, 1, 9];
/// bubble_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 5, 8, 9]);
/// ```
///
/// ```
/// use exp_sort::bubble_sort;
///
/// let mut arr = [10, 4, 7, 3, 1, 8, 2, 9, 5, 6, 15, 12, 18, 11, 14, 17, 13, 16, 19, 1];
/// bubble_sort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
/// ```
/// small - time:   [3.9278 ns 3.9323 ns 3.9369 ns]
/// big - time:   [370.78 µs 375.28 µs 379.72 µs]
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for a in 0..len - 1 - i {
            if arr[a] > arr[a + 1] {
                arr.swap(a, a + 1);
            }
        }
    }
}

/// Сортировка выбором (Selection Sort)
/// Алгоритм работает путем нахождения минимального элемента в неотсортированной части массива
/// и обмена его с первым элементом неотсортированной части. Так происходит поочередно для каждого
/// элемента. Имеет временную сложность O(n²), но обычно работает быстрее сортировки пузырьком,
/// так как делает меньше обменов(на больших массивах).
///
///
/// # Примеры
///
/// ```
/// use exp_sort::selection_sort;
///
/// let mut arr = [2, 3, 1];
/// selection_sort(&mut arr);
/// assert_eq!(arr, [1, 2 ,3]);
/// ```
///
/// ```
/// use exp_sort::selection_sort;
///
/// let mut arr = [5, 2, 8, 1, 9];
/// selection_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 5, 8, 9]);
/// ```
///
/// ```
/// use exp_sort::selection_sort;
///
/// let mut arr = [10, 4, 7, 3, 1, 8, 2, 9, 5, 6, 15, 12, 18, 11, 14, 17, 13, 16, 19, 1];
/// selection_sort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
/// ```
///
/// small - time:   [9.6398 ns 9.7356 ns 9.8355 ns]
/// big - time:   [232.99 µs 259.58 µs 286.13 µs]
///
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for current in 0..len {
        let mut min_index = current;
        for i in current + 1..len {
            if arr[i] < arr[min_index] {
                min_index = i;
            }
        }
        if min_index != current {
            arr.swap(current, min_index);
        }
    }
}

/// Сортировка вставками (Insertion Sort)
/// Эффективен для небольших и почти отсортированных массивов. Имеет временную сложность O(n²) в худшем случае,
/// но может работать быстрее на почти отсортированных данных. Алгоритм последовательно строит отсортированную часть массива,
/// вставляя каждый новый элемент на правильную позицию.
///
/// # Примеры
///
/// ```
/// use exp_sort::insertion_sort;
///
/// let mut arr = [2, 3, 1];
/// insertion_sort(&mut arr);
/// assert_eq!(arr, [1, 2 ,3]);
/// ```
///
/// ```
/// use exp_sort::insertion_sort;
///
/// let mut arr = [5, 2, 8, 1, 9];
/// insertion_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 5, 8, 9]);
/// ```
///
/// ```
/// use exp_sort::insertion_sort;
///
/// let mut arr = [10, 4, 7, 3, 1, 8, 2, 9, 5, 6, 15, 12, 18, 11, 14, 17, 13, 16, 19, 1];
/// insertion_sort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
/// ```
///
/// small - time:   [3.2941 ns 3.2994 ns 3.3058 ns]
/// big - time:   [98.217 µs 98.390 µs 98.560 µs]
///
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    for current in 1..len {
        for index in 0..current {
            if arr[current] >= arr[index] {
                continue;
            }

            arr.swap(current, index);
        }
    }
}

/// Быстрая сортировка (Quick Sort)
/// Эффективный алгоритм со средней сложностью O(n log n). Реализуется рекурсивно.
/// Основная идея алгоритма — выбрать опорный элемент (pivot) и разделить массив на две части:
/// элементы меньше опорного и элементы больше или равные опорному.
/// Затем рекурсивно применяется сортировка к обеим частям.
///
/// # Примеры
///
/// ```
/// use exp_sort::quick_sort;
///
/// let mut arr = [2, 3, 1];
/// quick_sort(&mut arr);
/// assert_eq!(arr, [1, 2 ,3]);
/// ```
///
/// ```
/// use exp_sort::quick_sort;
///
/// let mut arr = [5, 2, 8, 1, 9];
/// quick_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 5, 8, 9]);
/// ```
///
/// ```
/// use exp_sort::quick_sort;
///
/// let mut arr = [10, 4, 7, 3, 1, 8, 2, 9, 5, 6, 15, 12, 18, 11, 14, 17, 13, 16, 19, 1];
/// quick_sort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
/// ```
///
/// small - [12.039 ns 12.061 ns 12.085 ns]
/// big - [30.098 µs 30.134 µs 30.167 µs]
///
pub fn quick_sort<T: Ord + Copy + Debug>(arr: &mut [T]) {
    if arr.len() > 1 {
        quick_sort_helper(arr, 0, arr.len() - 1);
    }
}

fn quick_sort_helper<T: Ord + Copy + Debug>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let pi = quick_sort_partition(arr, low, high);
        if pi > 0 {
            quick_sort_helper(arr, low, pi.wrapping_sub(1));
        }
        quick_sort_helper(arr, pi + 1, high);
    }
}

fn quick_sort_partition<T: Ord + Copy + Debug>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

/// Сортировка слиянием (Merge Sort)
/// Стабильный алгоритм с гарантированной сложностью O(n log n).
/// Работает по принципу "разделяй и властвуй": массив рекурсивно делится на две части,
/// каждая часть сортируется отдельно, а затем объединяется в один отсортированный массив.
/// Подходит для больших массивов и гарантирует стабильную производительность.
///
/// # Примеры
///
/// ```
/// use exp_sort::merge_sort;
///
/// let mut arr = [2, 3, 1];
/// merge_sort(&mut arr);
/// assert_eq!(arr, [1, 2 ,3]);
/// ```
///
/// ```
/// use exp_sort::merge_sort;
///
/// let mut arr = [5, 2, 8, 1, 9];
/// merge_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 5, 8, 9]);
/// ```
///
/// ```
/// use exp_sort::merge_sort;
///
/// let mut arr = [10, 4, 7, 3, 1, 8, 2, 9, 5, 6, 15, 12, 18, 11, 14, 17, 13, 16, 19, 1];
/// merge_sort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
/// ```
///
/// small - time:   [58.488 ns 58.561 ns 58.640 ns]
/// big - time:   [20.218 µs 20.283 µs 20.345 µs]
///
pub fn merge_sort<T: Ord + Copy + Debug>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut merge = Vec::with_capacity(len);
    let (left, right) = arr.split_at(mid);

    let left_len = left.len();
    let right_len = right.len();

    let mut a = 0;
    let mut b = 0;

    while a < left_len && b < right_len {
        if left[a] <= right[b] {
            merge.push(left[a]);
            a += 1;
        } else {
            merge.push(right[b]);
            b += 1;
        };
    }

    for value in left.iter().skip(a) {
        merge.push(*value);
    }
    for value in right.iter().skip(b) {
        merge.push(*value);
    }

    arr.copy_from_slice(&merge);
}
