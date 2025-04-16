//! Цель создать бесконечный интератор с рандомными числами
//!
//! # Использование через функцию
//!
//! ```no_run
//! use rand_iterator::rand_iter;
//!
//! for (num,value) in rand_iter(-128..128).enumerate().take(50) {
//!    println!("{num}#: {value}")
//! }
//! ```
//!
//! # Использование через макрос
//!
//! ```no_run
//! use rand_iterator::rand;
//!
//! for (num,value) in rand!(-128..128, 50).enumerate() {
//!     println!("{num}#: {value}")
//! }
//!
//! rand!().take(50).enumerate().for_each(|(num,value)| println!("{num}#: {value}"));
//! ```
//!
//! # Использование через трейт
//!
//! ```no_run
//!
//! ```
//!

pub mod function;
pub mod macros;

pub use function::rand_iter;
