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
//! Мои ожидания были что for будет быстрее чем рекурсия.
//! Похоже чем примитивнее код тем быстрее будет его выполнение. А рекурсия примитивнее чем итератор.
//!
//! Топ по скорости выполнения:
//! 1. `cycle_loop`
//! 2. `cycle_while`
//! 3. `fast_recursion`
//! 4. `fast_cycle_for`
//! 5. `iterator_fold`
//!
//! Остался последний момент. Сделать функцию безопаснее и избавиться от overflow.
//! Для этого возьмем самый быстрый вариант и добавить безопасное сложение.
//!
//! Самым быстрым вариантом добавить ограничение на вводные данные количества итераций
//! и если будет превышать то возвращать 0.
//!
//! ```no_run
//! use fibonacci::MAX_FIBONACCI_FOR_U64;
//!
//! pub fn safe_fibonacci_v1(mut n: u64) -> u64 {
//!    if MAX_FIBONACCI_FOR_U64 < n {
//!        return 0;
//!    }
//!
//!    if n <= 1 {
//!        return 1;
//!    }
//!
//!    let mut a: u64 = 0;
//!    let mut b: u64 = 1;
//!
//!    loop {
//!        n -= 1;
//!        let c = a + b;
//!        a = b;
//!        b = c;
//!        if n == 0 {
//!            break;
//!        }
//!    }
//!
//!    b
//! }
//!
//! ```
//! **Benchmarks:**
//! - 92 - [8.0395 ns 8.0559 ns 8.0797 ns]
//!
//! Но у такого решения есть огромный недостаток. Ответ 0 не очевиден
//! и при использования этой функции возможна человеческая ошибка.
//!
//! Переделаем на возвращение ошибки через `Result`.
//! ```no_run
//! use fibonacci::MAX_FIBONACCI_FOR_U64;
//!
//! pub fn safe_fibonacci_v2(mut n: u64) -> Result<u64, u8> {
//!     if MAX_FIBONACCI_FOR_U64 < n {
//!         return Err(0);
//!     }
//!
//!     if n <= 1 {
//!         return Ok(1);
//!     }
//!
//!     let mut a: u64 = 0;
//!     let mut b: u64 = 1;
//!
//!     loop {
//!         n -= 1;
//!         let c = a + b;
//!         a = b;
//!         b = c;
//!         if n == 0 {
//!             break;
//!         }
//!     }
//!
//!     Ok(b)
//! }
//! ```
//!
//! **Benchmarks:**
//! - 92 - [8.4858 ns 8.4916 ns 8.4975 ns]
//!
//! Как видим за Result нужно платить.
//!
//! Попробуем добавить универсальности функции. Возможность принимать любой числовой тип.
//!
//! ```no_run
//! use fibonacci::MAX_FIBONACCI_FOR_U64;
//! use std::ops::SubAssign;
//!
//! pub fn safe_fibonacci_v3<N>(mut n: N) -> Result<u64, u8>
//! where
//!     N: PartialEq + PartialOrd + SubAssign + Copy + TryFrom<u8> + TryInto<u64>,
//! {
//!     if MAX_FIBONACCI_FOR_U64 < n.try_into().map_err(|_| 2u8)? {
//!         return Err(0);
//!     }
//!
//!     let min_iter: N = 1_u8.try_into().map_err(|_| 3u8)?;
//!     let finish_inter: N = 0_u8.try_into().map_err(|_| 4u8)?;
//!
//!     if n <= min_iter {
//!         return Ok(1);
//!     }
//!
//!     let mut a: u64 = 0;
//!     let mut b: u64 = 1;
//!
//!     loop {
//!         n -= min_iter;
//!         let c = a + b;
//!         a = b;
//!         b = c;
//!         if n == finish_inter {
//!             break;
//!         }
//!     }
//!
//!     Ok(b)
//! }
//! ```
//!
//! **Benchmarks:**
//! - 92 - [8.4963 ns 8.5032 ns 8.5105 ns]
//!
//! За конвертацию типов и дополнительные обработки ошибок нужно платить. Но показатели были измены не значительно.
//!
//!
//! Можно избавиться от проверки на максимальное количество итераций, заменив на безопасное сложение.
//!
//! ```no_run
//! use std::ops::SubAssign;
//!
//! pub fn safe_fibonacci_v4<N>(mut n: N) -> Result<u64, u8>
//! where
//!     N: PartialEq + PartialOrd + SubAssign + Copy + TryFrom<u8> + TryInto<u64>,
//! {
//!     let min_iter: N = 1_u8.try_into().map_err(|_| 3u8)?;
//!     let finish_inter: N = 0_u8.try_into().map_err(|_| 4u8)?;
//!
//!     if n <= min_iter {
//!         return Ok(1);
//!     }
//!
//!     let mut a: u64 = 0;
//!     let mut b: u64 = 1;
//!
//!     loop {
//!         n -= min_iter;
//!         let c = a.checked_add(b).ok_or(5u8)?;
//!         a = b;
//!         b = c;
//!         if n == finish_inter {
//!             break;
//!         }
//!     }
//!
//!     Ok(b)
//! }
//! ```
//! **Benchmarks:**
//! - 92 - [25.803 ns 26.058 ns 26.340 ns]
//!
//! Как видим это дорогая операция и её лучше применять там где результат невозможно просчитать.  
//!
//!

use std::ops::SubAssign;

pub mod cycle;
pub mod iterator;
pub mod recursion;

pub const MAX_FIBONACCI_FOR_U64: u64 = 92;

/// Безопасная реализация функции для вычисления чисел Фибоначчи
/// Защита через константу максимально допустимых количества итераций.
/// Если количество итераций больше максимально допустимого то вернет `0`.
///
/// ```rust
/// use fibonacci::{safe_fibonacci_v1,MAX_FIBONACCI_FOR_U64};
/// assert_eq!(safe_fibonacci_v1(0),1);
/// assert_eq!(safe_fibonacci_v1(1),1);
/// assert_eq!(safe_fibonacci_v1(11),144);
/// assert_eq!(safe_fibonacci_v1(22),28657);
/// assert_eq!(safe_fibonacci_v1(MAX_FIBONACCI_FOR_U64),12200160415121876738);
/// assert_eq!(safe_fibonacci_v1(MAX_FIBONACCI_FOR_U64+1),0);
/// ```
///
/// **Benchmarks:**
/// - 92 - [8.0395 ns 8.0559 ns 8.0797 ns]
pub fn safe_fibonacci_v1(mut n: u64) -> u64 {
    if MAX_FIBONACCI_FOR_U64 < n {
        return 0;
    }

    if n <= 1 {
        return 1;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    loop {
        n -= 1;
        let c = a + b;
        a = b;
        b = c;
        if n == 0 {
            break;
        }
    }

    b
}

/// Безопасная реализация функции для вычисления чисел Фибоначчи
/// Защита через константу максимально допустимых количества итераций.
/// Если количество итераций больше максимально допустимого то вернет Err(0).
///
/// ```rust
/// use fibonacci::{safe_fibonacci_v2,MAX_FIBONACCI_FOR_U64};
/// assert_eq!(safe_fibonacci_v2(0),Ok(1));
/// assert_eq!(safe_fibonacci_v2(1),Ok(1));
/// assert_eq!(safe_fibonacci_v2(11),Ok(144));
/// assert_eq!(safe_fibonacci_v2(22),Ok(28657));
/// assert_eq!(safe_fibonacci_v2(MAX_FIBONACCI_FOR_U64),Ok(12200160415121876738));
/// assert!(safe_fibonacci_v2(MAX_FIBONACCI_FOR_U64+1).is_err());
/// ```
///
/// **Benchmarks:**
/// - 92 - [8.4858 ns 8.4916 ns 8.4975 ns]
pub fn safe_fibonacci_v2(mut n: u64) -> Result<u64, u8> {
    if MAX_FIBONACCI_FOR_U64 < n {
        return Err(0);
    }

    if n <= 1 {
        return Ok(1);
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    loop {
        n -= 1;
        let c = a + b;
        a = b;
        b = c;
        if n == 0 {
            break;
        }
    }

    Ok(b)
}

/// Безопасная реализация функции для вычисления чисел Фибоначчи
/// ```rust
/// use fibonacci::{safe_fibonacci_v3,MAX_FIBONACCI_FOR_U64};
/// assert_eq!(safe_fibonacci_v3(0_i8),Ok(1));
/// assert_eq!(safe_fibonacci_v3(1_u16),Ok(1));
/// assert_eq!(safe_fibonacci_v3(11_i32),Ok(144));
/// assert_eq!(safe_fibonacci_v3(22_isize),Ok(28657));
/// assert_eq!(safe_fibonacci_v3(MAX_FIBONACCI_FOR_U64),Ok(12200160415121876738));
/// assert!(safe_fibonacci_v3(MAX_FIBONACCI_FOR_U64+1).is_err());
/// ```
///
/// **Benchmarks:**
/// - 92 - [8.4963 ns 8.5032 ns 8.5105 ns]
pub fn safe_fibonacci_v3<N>(mut n: N) -> Result<u64, u8>
where
    N: PartialEq + PartialOrd + SubAssign + Copy + TryFrom<u8> + TryInto<u64>,
{
    if MAX_FIBONACCI_FOR_U64 < n.try_into().map_err(|_| 2u8)? {
        return Err(0);
    }

    let min_iter: N = 1_u8.try_into().map_err(|_| 3u8)?;
    let finish_inter: N = 0_u8.try_into().map_err(|_| 4u8)?;

    if n <= min_iter {
        return Ok(1);
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    loop {
        n -= min_iter;
        let c = a + b;
        a = b;
        b = c;
        if n == finish_inter {
            break;
        }
    }

    Ok(b)
}

/// Безопасная реализация функции для вычисления чисел Фибоначчи
/// ```rust
/// use fibonacci::{safe_fibonacci_v4,MAX_FIBONACCI_FOR_U64};
/// assert_eq!(safe_fibonacci_v4(0_i8),Ok(1));
/// assert_eq!(safe_fibonacci_v4(1_u16),Ok(1));
/// assert_eq!(safe_fibonacci_v4(11_i32),Ok(144));
/// assert_eq!(safe_fibonacci_v4(22_isize),Ok(28657));
/// assert_eq!(safe_fibonacci_v4(MAX_FIBONACCI_FOR_U64),Ok(12200160415121876738));
/// assert!(safe_fibonacci_v4(MAX_FIBONACCI_FOR_U64+1).is_err());
/// ```
///
/// **Benchmarks:**
/// - 92 - [25.803 ns 26.058 ns 26.340 ns]
pub fn safe_fibonacci_v4<N>(mut n: N) -> Result<u64, u8>
where
    N: PartialEq + PartialOrd + SubAssign + Copy + TryFrom<u8> + TryInto<u64>,
{
    let min_iter: N = 1_u8.try_into().map_err(|_| 3u8)?;
    let finish_inter: N = 0_u8.try_into().map_err(|_| 4u8)?;

    if n <= min_iter {
        return Ok(1);
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    loop {
        n -= min_iter;
        let c = a.checked_add(b).ok_or(5u8)?;
        a = b;
        b = c;
        if n == finish_inter {
            break;
        }
    }

    Ok(b)
}

#[cfg(test)]
mod tests {
    use crate::{
        cycle::{
            cycle_for_v1, cycle_for_v3, cycle_for_v4, cycle_for_v5, cycle_for_v6, cycle_loop,
            cycle_while, fast_cycle_for,
        },
        iterator::iterator_fold,
        recursion::*,
        safe_fibonacci_v1, safe_fibonacci_v2, safe_fibonacci_v3, safe_fibonacci_v4,
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
    fn check_with_result(f: impl Fn(u64) -> Result<u64, u8>) {
        assert_eq!(Ok(VALID_RESULT[0]), f(0));
        assert_eq!(Ok(VALID_RESULT[1]), f(1));
        assert_eq!(Ok(VALID_RESULT[5]), f(5));
        assert_eq!(Ok(VALID_RESULT[10]), f(10));
        assert_eq!(Ok(12200160415121876738), f(MAX_FIBONACCI_FOR_U64));
        assert!(f(MAX_FIBONACCI_FOR_U64 + 1).is_err());
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
            safe_fibonacci_v1,
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

    #[test]
    fn test_safe_fibonacci_v1_max() {
        assert_eq!(safe_fibonacci_v1(MAX_FIBONACCI_FOR_U64 + 1), 0);
    }

    #[test]
    fn test_safe_fibonacci() {
        [safe_fibonacci_v2, safe_fibonacci_v3, safe_fibonacci_v4]
            .iter()
            .for_each(check_with_result);
    }

    #[test]
    fn test_safe_fibonacci_v3() {
        assert_eq!(safe_fibonacci_v3(10u8), Ok(VALID_RESULT[10]));
        assert_eq!(safe_fibonacci_v3(10u32), Ok(VALID_RESULT[10]));
        assert_eq!(safe_fibonacci_v3(10i8), Ok(VALID_RESULT[10]));
        assert_eq!(safe_fibonacci_v3(10isize), Ok(VALID_RESULT[10]));
    }

    #[test]
    fn test_safe_fibonacci_v4() {
        assert_eq!(safe_fibonacci_v4(10u8), Ok(VALID_RESULT[10]));
        assert_eq!(safe_fibonacci_v4(10u32), Ok(VALID_RESULT[10]));
        assert_eq!(safe_fibonacci_v4(10i8), Ok(VALID_RESULT[10]));
        assert_eq!(safe_fibonacci_v4(10isize), Ok(VALID_RESULT[10]));
    }
}
