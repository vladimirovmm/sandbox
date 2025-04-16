//! Цель создать трайт для Range, который добавит метод для создания интератора с рандомными числами.
//!
//! Пример:
//!
use rand::distr::uniform::SampleRange;

/// ```rust
/// use rand_iterator::RandIterator;
///
/// assert!((20..30).rand_iter().take(10).max().unwrap()<30_u8);
/// assert!((20..30).rand_iter().take(20).min().unwrap()>=20_i8);
/// ```
///
use crate::rand_iter;

pub trait RandIterator<N> {
    fn rand_iter(&self) -> impl Iterator<Item = N>;
}

impl<N, T> RandIterator<N> for T
where
    N: rand::distr::uniform::SampleUniform + Clone + PartialOrd,
    T: SampleRange<N> + Clone,
{
    /// Метод для создания интератора с рандомными числами по этому диапазону.
    ///
    /// ```rust
    /// use rand_iterator::RandIterator;
    ///
    /// let rand_vec: Vec<u16> = (200..=300).rand_iter().take(10).collect();
    /// assert_eq!(rand_vec.len(), 10);
    /// println!("random vec: {rand_vec:#?}");
    /// ```
    ///
    fn rand_iter(&self) -> impl Iterator<Item = N> {
        rand_iter(self.clone())
    }
}
