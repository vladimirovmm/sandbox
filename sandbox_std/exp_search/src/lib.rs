//! Алгоритмы поиска
//! В данном файле рассмотрены самые популярные алгоритмы поиска,
//! протестированы на скорость исполнения и реализованы на языке Rust.
//!
//! @todo сравнить со стандартным в rust

/// Линейный поиск (Linear Search)
/// Характеристики:
///  * Сложность по времени: O(n).
///  * Сложность по памяти: O(1).
///  * Работает с любыми данными, не требует сортировки.
///
///
/// # Примеры
///
/// ```
/// use exp_search::linear_search;
///
/// let arr = [1, 2, 3, 4, 5];
/// let target = 3;
/// assert_eq!(linear_search(&arr, &target), Some(2));
/// ```
///
/// ```
/// use exp_search::linear_search;
///
/// let arr = ["a", "b", "c"];
/// let target = "d";
/// assert_eq!(linear_search(&arr, &target), None);
/// ```
///
/// ```
/// use exp_search::linear_search;
///
/// let arr = [10, 20, 30];
/// let target = 10;
/// assert_eq!(linear_search(&arr, &target), Some(0));
/// ```
///
pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    for (index, element) in arr.iter().enumerate() {
        if element == target {
            return Some(index);
        }
    }
    None
}

/// Бинарный поиск (Binary Search)
/// Принцип работы: работает только с отсортированными массивами. На каждом шаге делит диапазон поиска пополам, сравнивая средний элемент с искомым.
/// Когда использовать:
///  * массив отсортирован;
///  * требуется часто выполнять поиск;
///  * нужна высокая производительность.
///
/// Характеристики:
///  * Сложность по времени: O(log n).
///  * Сложность по памяти: O(1) для итеративной версии, O(log n) для рекурсивной.
///  * Требование: массив должен быть отсортирован.
///
/// ```
/// use exp_search::binary_search;
///
/// let arr = [1, 2, 3, 4, 5];
/// let target = 3;
/// assert_eq!(binary_search(&arr, &target), Some(2));
/// ```
///
/// Не сработает с неотсортированным массивом
///
/// ```
/// use exp_search::binary_search;
///
/// let arr = [3, 5, 1, 2, 4];
/// let target = 3;
/// assert_eq!(binary_search(&arr, &target), None);
/// ```
///
pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }

    None
}
