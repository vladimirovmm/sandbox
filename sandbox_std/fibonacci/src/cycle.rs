//!
//! Подсчёта числа Фибоначчи через цикл for + swap
//!
//! ```no_run
//! pub fn fast_cycle_for(n: u64) -> u64 {
//!     if n <= 1 {
//!         return 1;
//!     }
//!
//!     let mut a = 0;
//!     let mut b = 1;
//!
//!     for _ in 0..n {
//!         a += b;
//!         std::mem::swap(&mut a, &mut b);
//!     }
//!
//!     b
//! }
//! ```
//!
//! **Benchmarks:**
//! - 20 - [2.9435 ns 2.9516 ns 2.9619 ns]
//! - 50 - [3.7977 ns 3.8021 ns 3.8065 ns]
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
///
pub fn cycle_for_v1(n: u64) -> u64 {
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
/// use fibonacci::cycle::cycle_for_v2;
/// assert_eq!(cycle_for_v2(0),1);
/// assert_eq!(cycle_for_v2(1),1);
/// assert_eq!(cycle_for_v2(11),144);
/// assert_eq!(cycle_for_v2(22),28657);
/// ```
///
/// **Benchmarks:**
/// - 20 - [2.7657 ns 2.7692 ns 2.7726 ns]
/// - 50 - [3.8084 ns 3.8139 ns 3.8197 ns]
///
pub fn cycle_for_v2(n: u64) -> u64 {
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
///
pub fn cycle_for_v3(n: u64) -> u64 {
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
///
pub fn cycle_for_v4(n: u64) -> u64 {
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
///
pub fn cycle_for_v5(n: u64) -> u64 {
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
/// use fibonacci::cycle::fast_cycle_for;
/// assert_eq!(fast_cycle_for(0),1);
/// assert_eq!(fast_cycle_for(1),1);
/// assert_eq!(fast_cycle_for(11),144);
/// assert_eq!(fast_cycle_for(22),28657);
/// ```
///
/// **Benchmarks:**
/// - 20 - [2.7657 ns 2.7692 ns 2.7726 ns]
/// - 50 - [3.7774 ns 3.7826 ns 3.7882 ns]
///
pub fn fast_cycle_for(n: u64) -> u64 {
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
