//! Цель создать бесконечный интератор с рандомными числами
//!
//! # Использование функций
//!
//! ```no_run
//! use rand_iterator::rand_iter;
//!
//! for (num,value) in rand_iter(-128..128).enumerate().take(50) {
//!    println!("{num}#: {value}")
//! }
//! ```
//!
//! # Макрос
//!
//! @todo
//!
//!

pub mod func;
pub mod macr;

pub use func::rand_iter;
