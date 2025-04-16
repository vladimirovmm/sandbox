//! Цель создать макрос который будет возвращать интератор содержащий рандомные числа.
//! Взможно задать деапазон рандомного числа и количество чисел.
//!
//! # Примеры
//!
//! ```rust
//! use rand_iterator::rand;
//!
//! let max = rand!().take(100).max().unwrap();
//! println!("Максимальное число в рандоме: {max}");
//!
//! let min = rand!(-2..10).take(100).min().unwrap();
//! println!("Минимальное число в рандоме: {min}");
//!
//! let sum = rand!(0..10, 10).sum::<i32>();
//! println!("Сумма рандомных чисел: {sum}");
//! ```
//!
//!

/// Макрос который возвращает итератор с рандомными числами
///
/// # Пример без параметров. Дефолтный диапазон (i32::MIN..i32::MAX)
///
/// ```rust
/// use rand_iterator::rand;
///
/// assert!(rand!().take(1000).max().unwrap() < i32::MAX);
/// ```
///
/// # Пример с заданным диапазоном
///
/// ```rust
/// use rand_iterator::rand;
/// assert!(rand!(-2..2).take(100).max().unwrap() < 2);
/// ```
///
/// # Пример с заданным количеством рандомных чисел
///
/// ```rust
/// use rand_iterator::rand;
/// assert_eq!(rand!(-2..2, 10).count(), 10);
/// ```
///
#[macro_export]
macro_rules! rand {
    () => {
        $crate::rand_iter(i32::MIN..i32::MAX)
    };
    ( $range:expr) => {
        $crate::rand_iter($range)
    };
    ( $range:expr, $len:expr ) => {
        $crate::rand_iter($range).take($len)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macros() {
        assert!(rand!(-2..2, 10).max().unwrap() < 2);
        assert!(rand!(-2..2, 10).min().unwrap() >= -2);
        assert_eq!(rand!(-2..2, 10).count(), 10);

        for (i, value) in rand!().take(10).enumerate() {
            println!("{i}#: {value}");
        }
    }
}
