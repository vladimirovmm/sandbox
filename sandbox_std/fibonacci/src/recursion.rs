//! Реализация подсчёта чисел Фибоначчи с помощью рекурсии
//!
//! Возьмём пример из из библиотеки [criterion](https://bheisler.github.io/criterion.rs/book/getting_started.html#getting-started)
//! и попробуем его оптимизировать.
//!
//! ```no_run
//! pub fn slow_recursion(n: u8) -> u64 {
//!    match n {
//!        0 => 1,
//!        1 => 1,
//!        n => slow_recursion(n - 1) + slow_recursion(n - 2),
//!    }
//! }
//! ```
//! Самая медленная реализация. Причина медленности - почти в каждом вызове функции выполняется два рекурсивных вызова.
//!
//! **Benchmarks:**
//!  - 20 - [12.718 µs 12.728 µs 12.740 µs]
//!  - 92 - Обработка занимает слишком много времени
//!
//! Попробуем оптимизировать ее. Для каждого вызова функции выполняется только один рекурсивный вызов.
//!
//! ```no_run
//! pub fn fast_recursion(n: u8) -> u64 {
//!    fn rec_fn(a: u64, b: u64, limit: u8) -> u64 {
//!        if limit == 0 {
//!            return b;
//!        }
//!        let c = b;
//!        let b = a + b;
//!        rec_fn(c, b, limit - 1)
//!    }
//!
//!    match n {
//!        0 => 1,
//!        1 => 1,
//!        n => rec_fn(0, 1, n),
//!    }
//! }
//! ```
//!
//! **Benchmarks:**
//!  - 20 - [2.6541 ns 2.6593 ns 2.6648 ns]
//!  - 50 - [3.7843 ns 3.7957 ns 3.8086 ns]
//!  - 92 - [8.1819 ns 8.1912 ns 8.2015 ns]
//!
//! Скорость выполнения для значения 20 увеличилась почти в 5 раз и появилась возможность подсчёта максимального значения.
//!
//! Попробуем избавиться от создания переменной `c`.
//!
//! ```no_run
//! fn recursion_v3(n: u8) -> u64 {
//!     fn rec_fn(a: u64, b: u64, limit: u8) -> u64 {
//!         if limit == 0 {
//!             return b;
//!         }
//!
//!         rec_fn(b, a + b, limit - 1)
//!     }
//!
//!     match n {
//!         0 => 1,
//!         1 => 1,
//!         n => rec_fn(0, 1, n),
//!     }
//! }
//! ```
//!
//! **Benchmarks:**
//! - 20 - [2.6289 ns 2.6333 ns 2.6383 ns]
//! - 50 - [3.7752 ns 3.7826 ns 3.7907 ns]
//! - 92 - [8.2130 ns 8.2199 ns 8.2280 ns]
//!
//! Можно попробовать уменьшить на один запуск функции.
//!
//! ```no_run
//! pub fn recursion_v4(n: u8) -> u64 {
//!     fn rec_fn(a: u64, b: u64, limit: u8) -> u64 {
//!         if limit == 1 {
//!             return a + b;
//!         }
//!
//!         rec_fn(b, a + b, limit - 1)
//!     }
//!
//!     match n {
//!         0 => 1,
//!         1 => 1,
//!         n => rec_fn(0, 1, n),
//!     }
//! }
//! ```
//! **Benchmarks:**
//! - 20 - [2.7953 ns 2.8033 ns 2.8118 ns]
//! - 50 - [3.7899 ns 3.7976 ns 3.8047 ns]
//! - 92 - [8.2729 ns 8.2793 ns 8.2856 ns]
//!
//! На удивление оказалось, что такой способ показал хуже результатов чем предыдущий. Скорее всего это связано с оптимизацией компилятора.
//! Но все результаты близки к друг другу.

/// Реализация через рекурсию из примера библиотеки [criterion](https://bheisler.github.io/criterion.rs/book/getting_started.html#getting-started)
/// ```rust
/// use fibonacci::recursion::slow_recursion;
/// assert_eq!(slow_recursion(0),1);
/// assert_eq!(slow_recursion(1),1);
/// assert_eq!(slow_recursion(11),144);
/// assert_eq!(slow_recursion(22),28657);
/// ```
pub fn slow_recursion(n: u8) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => slow_recursion(n - 1) + slow_recursion(n - 2),
    }
}

/// улучшенная версия `slow_recursion`. Для каждого вызова функции выполняется только один рекурсивный вызов.
///
/// ```rust
/// use fibonacci::recursion::fast_recursion;
/// assert_eq!(fast_recursion(0),1);
/// assert_eq!(fast_recursion(1),1);
/// assert_eq!(fast_recursion(11),144);
/// assert_eq!(fast_recursion(22),28657);
/// ```
pub fn fast_recursion(n: u8) -> u64 {
    fn rec_fn(a: u64, b: u64, limit: u8) -> u64 {
        if limit == 0 {
            return b;
        }
        let c = b;
        let b = a + b;
        rec_fn(c, b, limit - 1)
    }

    match n {
        0 => 1,
        1 => 1,
        n => rec_fn(0, 1, n),
    }
}

/// Отличие от fast_recursion - избавляемся от создания переменной `c`.
///
/// ```rust
/// use fibonacci::recursion::recursion_v3;
/// assert_eq!(recursion_v3(0),1);
/// assert_eq!(recursion_v3(1),1);
/// assert_eq!(recursion_v3(11),144);
/// assert_eq!(recursion_v3(22),28657);
/// ```
pub fn recursion_v3(n: u8) -> u64 {
    fn rec_fn(a: u64, b: u64, limit: u8) -> u64 {
        if limit == 0 {
            return b;
        }

        rec_fn(b, a + b, limit - 1)
    }

    match n {
        0 => 1,
        1 => 1,
        n => rec_fn(0, 1, n),
    }
}

/// Отличие от recursion_v3 - уменьшаем количество запусков на 1.
///
/// ```rust
/// use fibonacci::recursion::recursion_v4;
/// assert_eq!(recursion_v4(0),1);
/// assert_eq!(recursion_v4(1),1);
/// assert_eq!(recursion_v4(11),144);
/// assert_eq!(recursion_v4(22),28657);
/// ```
///
pub fn recursion_v4(n: u8) -> u64 {
    fn rec_fn(a: u64, b: u64, limit: u8) -> u64 {
        if limit == 1 {
            return a + b;
        }

        rec_fn(b, a + b, limit - 1)
    }

    match n {
        0 => 1,
        1 => 1,
        n => rec_fn(0, 1, n),
    }
}
