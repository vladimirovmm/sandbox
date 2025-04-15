//! ## Вводные данные:
//!
//! Чи́сла Фибона́ччи — элементы числовой последовательности:
//! ```text
//! 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, …,
//! ```
//! в которой первые два числа равны 0 и 1, а каждое последующее число равно сумме двух предыдущих чисел.
//! Названы в честь средневекового математика Леонардо Пизанского (известного как Фибоначчи).
//!
//! ### Реализация через рекурсии.
//!
//! Лучшие показатели для `fast_recursion`
//!
//! **Benchmarks:**
//!  - 20 - [2.6541 ns 2.6593 ns 2.6648 ns]
//!  - 50 - [3.7843 ns 3.7957 ns 3.8086 ns]
//!  - 92 - [8.1819 ns 8.1912 ns 8.2015 ns]
//!
//! ### Реализация через iterator.
//!
//! Лучшие показатели для `iterator_fold`
//!
//! **Benchmarks:**
//! - 20 - [3.2825 ns 3.2912 ns 3.3002 ns]
//! - 50 - [3.7567 ns 3.7652 ns 3.7746 ns]
//! - 92 - [8.2848 ns 8.2904 ns 8.2960 ns]
//!
//! ### Реализация через цикл
//!
//! ### for.
//!
//! Лучшие показатели для `fast_cycle_for`
//!
//! **Benchmarks:**
//! - 20 - [2.7657 ns 2.7692 ns 2.7726 ns]
//! - 50 - [3.8084 ns 3.8139 ns 3.8197 ns]
//! - 92 - [8.2685 ns 8.2738 ns 8.2791 ns]
//!
//! ### while.
//!
//! `cycle_while`
//!
//! **Benchmarks:**
//! - 92 -  [8.1015 ns 8.1075 ns 8.1135 ns]
//!
//! ### loop.
//!
//! `cycle_loop`
//!
//! **Benchmarks:**
//! - 92 - [8.0505 ns 8.0561 ns 8.0616 ns]
//!
//! ## Итог
//!
//! Результат оказался неожиданным.
//!
//! Топ по скорости выполнения:
//! 1. `cycle_loop`
//! 2. `cycle_while`
//! 3. `fast_recursion`
//! 4. `fast_cycle_for`
//! 5. `iterator_fold`
//!
//! Мои ожидания были что for будет быстрее чем рекурсия.
//! Похоже чем примитивнее код тем быстрее будет его выполнение. А рекурсия примитивнее чем итератор.
//!

pub mod cycle;
pub mod iterator;
pub mod recursion;

pub const MAX_FIBONACCI_FOR_U64: u64 = 92;

#[cfg(test)]
mod tests {
    use crate::{
        cycle::{
            cycle_for_v1, cycle_for_v3, cycle_for_v4, cycle_for_v5, cycle_for_v6, cycle_loop,
            cycle_while, fast_cycle_for,
        },
        iterator::iterator_fold,
        recursion::*,
        MAX_FIBONACCI_FOR_U64,
    };

    const VALID_RESULT: [u64; 22] = [
        1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        10946, 17711,
    ];

    fn check(f: impl Fn(u64) -> u64) {
        assert_eq!(VALID_RESULT[0], f(0));
        assert_eq!(VALID_RESULT[1], f(1));
        assert_eq!(VALID_RESULT[5], f(5));
        assert_eq!(VALID_RESULT[10], f(10));
        assert_eq!(VALID_RESULT[21], f(21));
    }

    #[test]
    fn test_fibonacci() {
        let array_with_fn = [
            slow_recursion,
            fast_recursion,
            recursion_v3,
            recursion_v4,
            iterator_fold,
            cycle_for_v1,
            fast_cycle_for,
            cycle_for_v3,
            cycle_for_v4,
            cycle_for_v5,
            cycle_for_v6,
            cycle_while,
            cycle_loop,
        ];
        array_with_fn.iter().for_each(check);

        // для `slow_recursion` нельзя передавать максимальное значение. Может привести к зависанию.
        // Поэтому пропускаем первый элемент
        array_with_fn.iter().skip(1).for_each(|f| {
            f(MAX_FIBONACCI_FOR_U64);
        });
    }

    #[test]
    #[should_panic]
    fn test_fibonacci_max() {
        cycle_for_v6(MAX_FIBONACCI_FOR_U64 + 1);
    }
}
