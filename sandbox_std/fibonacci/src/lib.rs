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
//! - 20 - [2.7093 ns 2.7128 ns 2.7164 ns]
//! - 50 - [3.7381 ns 3.7506 ns 3.7637 ns]
//!
//! ### Реализация через iterator.
//!
//! Лучшие показатели для `iterator_fold`
//!
//! **Benchmarks:**
//! - 20 - [3.2476 ns 3.2591 ns 3.2734 ns]
//! - 50 - [3.9065 ns 3.9321 ns 3.9539 ns]
//!
//! ### Реализация через цикл for.
//!
//! Лучшие показатели для `fast_cycle_for`
//!
//! **Benchmarks:**
//! - 20 - [2.9435 ns 2.9516 ns 2.9619 ns]
//! - 50 - [3.7977 ns 3.8021 ns 3.8065 ns]
//!
//! ## Итог
//!
//! Результат оказался неожиданым.
//! Я ожидал что `for` цикл вместе с будет быстрее всех. Но мне так не удалось добится этого.
//! Также ожидал что `iterator_fold` будет быстрее рекурсии.
//!
//! Цыфры блезки и возможно это результат rust оптимизатора.
//!

pub mod cycle;
pub mod iterator;
pub mod recursion;

#[cfg(test)]
mod tests {
    use crate::{
        cycle::{
            cycle_for_v1, cycle_for_v2, cycle_for_v3, cycle_for_v4, cycle_for_v5, fast_cycle_for,
        },
        iterator::iterator_fold,
        recursion::*,
    };

    const ANS: [u64; 22] = [
        1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        10946, 17711,
    ];

    fn check(f: impl Fn(u64) -> u64) {
        assert_eq!(ANS[0], f(0));
        assert_eq!(ANS[1], f(1));
        assert_eq!(ANS[5], f(5));
        assert_eq!(ANS[10], f(10));
        assert_eq!(ANS[21], f(21));
    }

    #[test]
    fn test_fibonacci() {
        [
            slow_recursion,
            recursion_v2,
            recursion_v3,
            fast_recursion,
            iterator_fold,
            cycle_for_v1,
            cycle_for_v2,
            cycle_for_v3,
            cycle_for_v4,
            cycle_for_v5,
            fast_cycle_for,
        ]
        .into_iter()
        .for_each(check);
    }
}
