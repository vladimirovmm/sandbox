//! Цель создать функцию возвращающая интератор с рандомными числами
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

use std::ops::Range;

use rand::distr::uniform::SampleRange;

/// Недостаток этого способа это ограниченность интераций.
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

/// Бесконечный интератор для u64, но с удобным обозначением границ через Range.
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
/// Пример
/// ```rust
/// use rand_iterator::rand_iter;
///
/// assert!(rand_iter(0..2).next().is_some());
/// ```
///
/// тест пример на кол-во элементов
/// ```rust
/// use rand_iterator::rand_iter;
///
/// assert_eq!(rand_iter(12..212).take(5).count(), 5);
/// ```
///
/// Тест пример на минимальнои и максимальное число
/// ```rust
/// use rand_iterator::rand_iter;
///
/// assert!(rand_iter(20..30).take(10).max().unwrap()<30_u8);
/// assert!(rand_iter(20..30).take(20).min().unwrap()>=20_i8);
/// ```
///
/// Пример вывода 5 элементов из созданного интератора
/// ```no_run
/// use rand_iterator::rand_iter;
///
/// for (num,value) in rand_iter(-1_000_000..100).enumerate().take(5) {
///    println!("{num}#: {value}")
/// }
/// ```
///
/// Пример создания интератора из 10 элементов со случайными числами из диапазона от 200 до 300.
/// ```rust
/// use rand_iterator::rand_iter;
///
/// let rand_vec: Vec<u16> = rand_iter(200..=300).take(10).collect();
/// assert_eq!(rand_vec.len(), 10);
/// println!("random vec: {rand_vec:#?}");
/// ```
///
pub fn rand_iter<N, R>(range: R) -> impl Iterator<Item = N>
where
    N: rand::distr::uniform::SampleUniform + Clone + PartialOrd,
    R: SampleRange<N> + Clone,
{
    std::iter::repeat_with(move || rand::random_range(range.clone()))
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;

    const INPUT_RANGE: Range<u64> = 0..100;
    const COUNT: usize = 100;

    fn check(it: impl Iterator<Item = u64>) {
        let numbs: Vec<u64> = it.take(COUNT).collect();

        assert_eq!(numbs.len(), COUNT);
        assert!(numbs.iter().min().unwrap() >= &INPUT_RANGE.start);
        assert!(numbs.iter().max().unwrap() < &INPUT_RANGE.end);
    }

    #[test]
    fn test_rand() {
        let Range { start, end } = INPUT_RANGE;
        check(rand_iter_v1(start, end));
        check(rand_iter_v2(start, end));
    }

    #[test]
    fn test_rand_with_range() {
        check(rand_iter_v3(INPUT_RANGE));
        check(rand_iter(INPUT_RANGE));
    }
}
