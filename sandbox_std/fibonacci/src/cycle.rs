//!
//! ## Подсчёта числа Фибоначчи
//!
//! Лучшие результаты
//!
//! ### цикл for
//!
//! ```no_run
//! pub fn fast_cycle_for(n: u8) -> u64 {
//!     if n <= 1 {
//!         return 1;
//!     }
//!
//!     let mut a = 0;
//!     let mut b = 1;
//!
//!     for _ in 0..n {
//!         let c = a + b;
//!         a = b;
//!         b = c;
//!     }
//!
//!     b
//! }
//! ```
//!
//! **Benchmarks:**
//! - 20 - [2.7657 ns 2.7692 ns 2.7726 ns]
//! - 50 - [3.8084 ns 3.8139 ns 3.8197 ns]
//! - 92 - [8.2847 ns 8.2889 ns 8.2935 ns]
//!
//! ### цикл while
//!
//! ```no_run
//! pub fn cycle_while(mut n: u8) -> u64 {
//!     if n <= 1 {
//!         return 1;
//!     }
//!
//!     let mut a = 0;
//!     let mut b = 1;
//!
//!     while n > 0 {
//!         n -= 1;
//!
//!         let c = a + b;
//!         a = b;
//!         b = c;
//!     }
//!
//!     b
//! }
//! ```
//!
//! **Benchmarks:**
//! - 92 -  [8.1015 ns 8.1075 ns 8.1135 ns]
//!
//!
//! ### цикл loop
//!
//! ```no_run
//! pub fn cycle_loop(mut n: u8) -> u64 {
//!     if n <= 1 {
//!         return 1;
//!     }
//!
//!     let mut a = 0;
//!     let mut b = 1;
//!
//!     loop {
//!         n -= 1;
//!
//!         let c = a + b;
//!         a = b;
//!         b = c;
//!         if n == 0 {
//!             break;
//!         }
//!     }
//!
//!     b
//! }
//! ```
//!
//! **Benchmarks:**
//! - 92 - [8.0505 ns 8.0561 ns 8.0616 ns]
//!

/// Реализация подсчёта числа Фибоначчи через цикл for
///
/// ```rust
/// use fibonacci::cycle::cycle_for_v1;
/// assert_eq!(cycle_for_v1(0),1);
/// assert_eq!(cycle_for_v1(1),1);
/// assert_eq!(cycle_for_v1(11),144);
/// assert_eq!(cycle_for_v1(22),28657);
/// ```
///
/// **Benchmarks:**
/// - 20 - [2.9212 ns 2.9301 ns 2.9401 ns]
/// - 50 - [3.7643 ns 3.7717 ns 3.7800 ns]
/// - 92 - [8.2994 ns 8.3083 ns 8.3168 ns]
///
pub fn cycle_for_v1(n: u8) -> u64 {
    match n {
        0 | 1 => return 1,
        _ => (),
    };

    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}

/// Реализация подсчёта числа Фибоначчи через цикл for
///
/// ```rust
/// use fibonacci::cycle::fast_cycle_for;
/// assert_eq!(fast_cycle_for(0),1);
/// assert_eq!(fast_cycle_for(1),1);
/// assert_eq!(fast_cycle_for(11),144);
/// assert_eq!(fast_cycle_for(22),28657);
/// ```
///
/// **Benchmarks:**
/// - 20 - [2.7657 ns 2.7692 ns 2.7726 ns]
/// - 50 - [3.8084 ns 3.8139 ns 3.8197 ns]
/// - 92 - [8.2847 ns 8.2889 ns 8.2935 ns]
///
pub fn fast_cycle_for(n: u8) -> u64 {
    if n <= 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}

/// Реализация подсчёта числа Фибоначчи через цикл for
///
/// ```rust
/// use fibonacci::cycle::cycle_for_v3;
/// assert_eq!(cycle_for_v3(0),1);
/// assert_eq!(cycle_for_v3(1),1);
/// assert_eq!(cycle_for_v3(11),144);
/// assert_eq!(cycle_for_v3(22),28657);
/// ```
///
/// **Benchmarks:**
/// - 20 - [2.8444 ns 2.8584 ns 2.8749 ns]
/// - 50 - [3.7269 ns 3.7333 ns 3.7400 ns]
/// - 92 - [8.3224 ns 8.3365 ns 8.3537 ns]
///
pub fn cycle_for_v3(n: u8) -> u64 {
    if matches!(n, 0 | 1) {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}

/// Реализация подсчёта числа Фибоначчи через цикл for
///
/// ```rust
/// use fibonacci::cycle::cycle_for_v4;
/// assert_eq!(cycle_for_v4(0),1);
/// assert_eq!(cycle_for_v4(1),1);
/// assert_eq!(cycle_for_v4(11),144);
/// assert_eq!(cycle_for_v4(22),28657);
/// ```
///
/// **Benchmarks:**
/// - 20 - [2.9419 ns 2.9477 ns 2.9533 ns]
/// - 50 - [4.2574 ns 4.2873 ns 4.3126 ns]
/// - 92 - [8.2829 ns 8.2893 ns 8.2962 ns]
///
pub fn cycle_for_v4(n: u8) -> u64 {
    match n {
        0 | 1 => return 1,
        _ => (),
    };

    let mut a = 0;
    let mut b = 1;
    let mut c;

    for _ in 0..n {
        c = a + b;
        a = b;
        b = c;
    }

    b
}

/// Реализация подсчёта числа Фибоначчи через цикл for
///
/// ```rust
/// use fibonacci::cycle::cycle_for_v5;
/// assert_eq!(cycle_for_v5(0),1);
/// assert_eq!(cycle_for_v5(1),1);
/// assert_eq!(cycle_for_v5(11),144);
/// assert_eq!(cycle_for_v5(22),28657);
/// ```
///
/// **Benchmarks:**
/// - 20 - [2.7657 ns 2.7692 ns 2.7726 ns]
/// - 50 - [3.7774 ns 3.7826 ns 3.7882 ns]
/// - 92 - [8.2898 ns 8.2958 ns 8.3021 ns]
///
pub fn cycle_for_v5(n: u8) -> u64 {
    match n {
        0 | 1 => return 1,
        _ => (),
    };

    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        a += b;
        std::mem::swap(&mut a, &mut b);
    }

    b
}

/// Реализация подсчёта числа Фибоначчи через цикл for + swap
///
/// ```rust
/// use fibonacci::cycle::cycle_for_v6;
/// assert_eq!(cycle_for_v6(0),1);
/// assert_eq!(cycle_for_v6(1),1);
/// assert_eq!(cycle_for_v6(11),144);
/// assert_eq!(cycle_for_v6(22),28657);
/// ```
///
/// **Benchmarks:**
/// - 20 - [2.7657 ns 2.7692 ns 2.7726 ns]
/// - 50 - [3.7774 ns 3.7826 ns 3.7882 ns]
/// - 92 - [8.2918 ns 8.2980 ns 8.3039 ns]
///
pub fn cycle_for_v6(n: u8) -> u64 {
    if n <= 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        a += b;
        std::mem::swap(&mut a, &mut b);
    }

    b
}

/// Реализация подсчёта числа Фибоначчи через цикл for
///
/// ```rust
/// use fibonacci::cycle::cycle_while;
/// assert_eq!(cycle_while(0),1);
/// assert_eq!(cycle_while(1),1);
/// assert_eq!(cycle_while(11),144);
/// assert_eq!(cycle_while(22),28657);
/// ```
///
/// **Benchmarks:**
/// - 92 -  [8.1015 ns 8.1075 ns 8.1135 ns]
///
pub fn cycle_while(mut n: u8) -> u64 {
    if n <= 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    while n > 0 {
        n -= 1;

        let c = a + b;
        a = b;
        b = c;
    }

    b
}

/// Реализация подсчёта числа Фибоначчи через цикл for
///
/// ```rust
/// use fibonacci::cycle::cycle_loop;
/// assert_eq!(cycle_loop(0),1);
/// assert_eq!(cycle_loop(1),1);
/// assert_eq!(cycle_loop(11),144);
/// assert_eq!(cycle_loop(22),28657);
/// ```
///
/// **Benchmarks:**
/// - 92 -  [8.0505 ns 8.0561 ns 8.0616 ns]
///
pub fn cycle_loop(mut n: u8) -> u64 {
    if n <= 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

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
