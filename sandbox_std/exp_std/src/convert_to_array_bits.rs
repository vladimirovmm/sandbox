//! Цель написать конвертер для приоброзвания числа в массив из битов
//!
//! # Реализация через форматирование
//! Самое простое в понимании чере format преобразавать значение в битовой вид
//!
//! ```rust
//! fn fmt_convert_v1<T>(number: T) -> Vec<bool>
//! where
//!     T: std::fmt::Binary,
//! {
//!     format!("{:b}", number).chars().map(|c| c == '1').collect()
//! }
//!
//! assert_eq!(fmt_convert_v1(1), vec![true]);
//! assert_eq!(fmt_convert_v1(2), vec![true,false]);
//! ```
//!
//! Что бы заполнить старшие нули можно использовать `std::mem::size_of_val(&value) * 8`
//!
//! ```rust
//! fn fmt_convert_v2<T>(number: T) -> Vec<bool>
//! where
//!     T: std::fmt::Binary,
//! {
//!     format!(
//!         "{number:0width$b}",
//!         width = std::mem::size_of_val(&number) * 8
//!     )
//!     .chars()
//!     .map(|c| c == '1')
//!     .collect()
//! }
//!
//! assert_eq!(fmt_convert_v2(u8::MAX), vec![true,true,true,true,true,true,true,true]);
//! assert_eq!(fmt_convert_v2(1_u8), vec![false,false,false,false,false,false,false,true]);
//! ```
//!
//! # Реализация через сдвиг
//!
//! ```no_run
//! fn shift_convert_v1(number: u8) -> Vec<bool> {
//!     (0..u8::BITS)
//!         .rev()
//!         .map(|index| (number >> index) & 1 == 1)
//!         .collect()
//! }
//!
//! ```
//! Недостаток этого метода, что он потерял возможность работать с другими типами.
//!

/// Преобразовать число в массив битов через форматирование
/// Недостаток этого метода, что он не отображает старшие нули.
///
/// ```rust
/// use exp_std::convert_to_array_bits::fmt_convert_v1;
/// assert_eq!(fmt_convert_v1(1), vec![true]);
/// assert_eq!(fmt_convert_v1(5), vec![true, false,true]);
/// ```
pub fn fmt_convert_v1<T>(number: T) -> Vec<bool>
where
    T: std::fmt::Binary,
{
    format!("{number:b}").chars().map(|c| c == '1').collect()
}

/// Преобразовать число в массив битов через форматирование.
/// Старшие биты будут заполнены нулями, а размер берётся из `std::mem::size_of_val(&value) * 8`
///
/// ```rust
/// use exp_std::convert_to_array_bits::fmt_convert_v2;
/// assert_eq!(fmt_convert_v2(u8::MAX), vec![true,true,true,true,true,true,true,true]);
/// assert_eq!(fmt_convert_v2(1_u8), vec![false,false,false,false,false,false,false,true]);
/// ```
pub fn fmt_convert_v2<T>(number: T) -> Vec<bool>
where
    T: std::fmt::Binary,
{
    format!(
        "{number:0width$b}",
        width = std::mem::size_of_val(&number) * 8
    )
    .chars()
    .map(|c| c == '1')
    .collect()
}

/// Преобразовать число в массив битов через сдвиг
/// Недостаток этого метода, что он потерял возможность работать с другими типами.
///
/// ```rust
/// use exp_std::convert_to_array_bits::shift_convert_v1;
/// assert_eq!(shift_convert_v1(1_u8), vec![false,false,false,false,false,false,false,true]);
/// assert_eq!(shift_convert_v1(u8::MAX), vec![true,true,true,true,true,true,true,true]);
/// ```
pub fn shift_convert_v1(number: u8) -> Vec<bool> {
    (0..u8::BITS)
        .rev()
        .map(|index| (number >> index) & 1 == 1)
        .collect()
}
