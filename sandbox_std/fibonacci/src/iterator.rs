//!
//! Подсчёта числа Фибоначчи через fold
//!
//! ```no_run
//! pub fn iterator_fold(n: u64) -> u64 {
//!     match n {
//!         0 | 1 => 1,
//!         n => (0..n).fold((0, 1), |(a, b), _| (b, a + b)).1,
//!     }
//! }
//! ```
//!
//! **Benchmarks:**
//! - 20 - [3.2825 ns 3.2912 ns 3.3002 ns]
//! - 50 - [3.7567 ns 3.7652 ns 3.7746 ns]
//!

/// Реализация подсчёта числа Фибоначчи через fold
///
/// ```rust
/// use fibonacci::iterator::iterator_fold;
/// assert_eq!(iterator_fold(0),1);
/// assert_eq!(iterator_fold(1),1);
/// assert_eq!(iterator_fold(11),144);
/// assert_eq!(iterator_fold(22),28657);
/// ```
pub fn iterator_fold(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => (0..n).fold((0, 1), |(a, b), _| (b, a + b)).1,
    }
}
