//! Цель создать интератор с рандомными числами
//!
//! Первое что приходит в голову использовать уже существующий интератор и подменить в них значения
//!
//! ```no_run
//! pub fn rand_iter_v1(min: u64, max: u64) -> impl Iterator<Item = u64> {
//!     (0..usize::MAX).map(move |_| rand::random_range(min..max))
//! }
//! ```
//!
//! **benchmark**
//! [410.26 ns 411.63 ns 413.29 ns]
//!
//! Но задача создать бесконечный интератор.
//!
//! Заменим (0..usize::MAX) на std::iter::repeat_with
//!
//! ```no_run
//! pub fn rand_iter_v2(min: u64, max: u64) -> impl Iterator<Item = u64> {
//!     std::iter::repeat_with(move || rand::random_range(min..max))
//! }
//! ```
//!
//! **benchmark**
//! [420.28 ns 421.48 ns 422.75 ns]
//!
//! Думаю было бы красивее если функция будет принимать Range
//!
//! ```no_run
//! use std::ops::Range;
//!
//! pub fn rand_iter_v3(range: Range<u64>) -> impl Iterator<Item = u64> {
//!     std::iter::repeat_with(move || rand::random_range(range.clone()))
//! }
//! ```
//!
//! **benchmark**
//! [415.56 ns 416.14 ns 416.61 ns]
//!
//! Осталось только добавить возможность генерировать не только u64
//!
//! ```no_run
//! use rand::distr::uniform::SampleUniform;
//! use std::ops::Range;
//!
//! pub fn rand_iter<N>(range: Range<N>) -> impl Iterator<Item = N>
//! where
//!    N: SampleUniform + Clone + PartialOrd,
//! {
//!    std::iter::repeat_with(move || rand::random_range(range.clone()))
//! }
//! ```
//!
//! **benchmark**
//! [413.00 ns 414.24 ns 415.39 ns]
//!

use rand::distr::uniform::SampleUniform;
use std::ops::Range;

/// Недостаток этого способа это ограниченость интераций.
///
/// **benchmark**
/// [410.26 ns 411.63 ns 413.29 ns]
///
pub fn rand_iter_v1(min: u64, max: u64) -> impl Iterator<Item = u64> {
    (0..usize::MAX).map(move |_| rand::random_range(min..max))
}

/// Бесконечный интератор, но только для u64.
///
/// **benchmark**
/// [420.28 ns 421.48 ns 422.75 ns]
///
pub fn rand_iter_v2(min: u64, max: u64) -> impl Iterator<Item = u64> {
    std::iter::repeat_with(move || rand::random_range(min..max))
}

/// Бесконечный интератор для u64 но с удобным обозначением границ через Range.
///
/// **benchmark**
/// [415.56 ns 416.14 ns 416.61 ns]
///
pub fn rand_iter_v3(range: Range<u64>) -> impl Iterator<Item = u64> {
    std::iter::repeat_with(move || rand::random_range(range.clone()))
}

/// Бесконечный интератор для всех типов
///
/// **benchmark**
/// [413.00 ns 414.24 ns 415.39 ns]
///
/// ```rust
/// use rand_iterator::rand_iter;
///
/// assert!(rand_iter(0..2).next().is_some());
/// assert_eq!(rand_iter(0..100).take(5).count(), 5);
///
/// for (num,value) in rand_iter(0..10).enumerate().take(5) {
///    println!("{num}#: {value}")
/// }
///
/// assert!(rand_iter(0..10).take(5).max().unwrap()<10);
///
/// let rand_vec: Vec<u8> = rand_iter(0..10).take(10).collect();
/// assert_eq!(rand_vec.len(), 10);
/// println!("random vec: {rand_vec:#?}");
/// ```
///
pub fn rand_iter<N>(range: Range<N>) -> impl Iterator<Item = N>
where
    N: SampleUniform + Clone + PartialOrd,
{
    std::iter::repeat_with(move || rand::random_range(range.clone()))
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use crate::{rand_iter, rand_iter_v1, rand_iter_v2, rand_iter_v3};

    const INPUT_RANGE: Range<u64> = 0..100;
    const COUNT: usize = 100;

    fn check(it: impl Iterator<Item = u64>) {
        let count = it
            .take(COUNT)
            .enumerate()
            .inspect(|(n, value)| println!("{n}#: {value}"))
            .count();

        assert_eq!(count, COUNT);
    }

    #[test]
    fn test_rand_iter_v1() {
        let Range { start, end } = INPUT_RANGE;
        check(rand_iter_v1(start, end));
    }

    #[test]
    fn test_rand_iter_v2() {
        let Range { start, end } = INPUT_RANGE;
        check(rand_iter_v2(start, end));
    }

    #[test]
    fn test_rand_with_range() {
        check(rand_iter_v3(INPUT_RANGE));
    }

    #[test]
    fn test_rand_iter() {
        check(rand_iter(INPUT_RANGE));
    }
}
