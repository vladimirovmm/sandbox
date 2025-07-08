//! Римские цифры обозначаются семью различными символами: I, V, X, L, C, D и M.
//! Значение символа
//!     I       1
//!     V       5
//!     X       10
//!     L       50
//!     C       100
//!     D       500
//!     M       1000
//! Например, римскими цифрами 2 записывается как II, просто две единицы складываются вместе. 12 записывается как XII, что означает просто X + II. Число 27 записывается как XXVII, что означает XX + V + II.
//!
//! Римские цифры обычно пишутся от наибольшего к наименьшему слева направо. Однако цифра, обозначающая четыре, не является IIII. Вместо этого число четыре записывается как IV. Поскольку единица стоит перед пятью, мы вычитаем ее, и получается четыре. Тот же принцип применим к числу девять, которое записывается как IX. Существует шесть случаев, когда используется вычитание:
//!
//! I можно поместить перед V (5) и X (10), чтобы получить 4 и 9.
//! X можно поместить перед L (50) и C (100), чтобы получить 40 и 90.
//! C можно поместить перед D (500) и M (1000), чтобы получить 400 и 900.
//! Получив римскую цифру, преобразуйте ее в целое число.
//!
//!
//! Example 1:
//!
//! Input: s = "III"
//! Output: 3
//! Explanation: III = 3.
//! Example 2:
//!
//! Input: s = "LVIII"
//! Output: 58
//! Explanation: L = 50, V= 5, III = 3.
//! Example 3:
//!
//! Input: s = "MCMXCIV"
//! Output: 1994
//! Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
//!  
//!
//! Ограничения:
//!
//! 1 <= сек.длина <= 15
//! s содержит только символы ('I', 'V', 'X', 'L', 'C', 'D', 'M').
//! Гарантируется, что s является допустимой римской цифрой в диапазоне [1, 3999].
#![cfg(test)]

use std::cmp::Ordering;

pub fn roman_to_int(s: String) -> i32 {
    let mut back_ch = 0;
    let mut result = 0;
    let mut buff = 0;
    for ch in s.chars() {
        let n = match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => {
                panic!("{ch} Unknown symbol");
            }
        };
        match back_ch.cmp(&n) {
            Ordering::Greater => {
                result += buff;
                buff = n;
            }
            Ordering::Less if back_ch != 0 => {
                result += n - buff;
                buff = 0;
            }
            _ => {
                buff += n;
            }
        }

        back_ch = n;
    }
    result + buff
}

#[test]
fn test_roman_to_int() {
    assert_eq!(1, roman_to_int("I".to_string()));
    assert_eq!(2, roman_to_int("II".to_string()));
    assert_eq!(3, roman_to_int("III".to_string()));
    assert_eq!(4, roman_to_int("IV".to_string()));
    assert_eq!(5, roman_to_int("V".to_string()));
    assert_eq!(6, roman_to_int("VI".to_string()));
    assert_eq!(7, roman_to_int("VII".to_string()));
    assert_eq!(8, roman_to_int("VIII".to_string()));
    assert_eq!(9, roman_to_int("IX".to_string()));
    assert_eq!(10, roman_to_int("X".to_string()));
    assert_eq!(11, roman_to_int("XI".to_string()));
    assert_eq!(12, roman_to_int("XII".to_string()));
    assert_eq!(13, roman_to_int("XIII".to_string()));
    assert_eq!(14, roman_to_int("XIV".to_string()));
    assert_eq!(15, roman_to_int("XV".to_string()));
    assert_eq!(16, roman_to_int("XVI".to_string()));
    assert_eq!(17, roman_to_int("XVII".to_string()));
    assert_eq!(18, roman_to_int("XVIII".to_string()));
    assert_eq!(19, roman_to_int("XIX".to_string()));
    assert_eq!(20, roman_to_int("XX".to_string()));
    assert_eq!(21, roman_to_int("XXI".to_string()));
    assert_eq!(22, roman_to_int("XXII".to_string()));
    assert_eq!(23, roman_to_int("XXIII".to_string()));
    assert_eq!(24, roman_to_int("XXIV".to_string()));
    assert_eq!(25, roman_to_int("XXV".to_string()));
    assert_eq!(26, roman_to_int("XXVI".to_string()));
    assert_eq!(27, roman_to_int("XXVII".to_string()));
    assert_eq!(28, roman_to_int("XXVIII".to_string()));
    assert_eq!(29, roman_to_int("XXIX".to_string()));
    assert_eq!(30, roman_to_int("XXX".to_string()));
    assert_eq!(40, roman_to_int("XL".to_string()));
    assert_eq!(58, roman_to_int("LVIII".to_string()));
    assert_eq!(90, roman_to_int("XC".to_string()));
    assert_eq!(500, roman_to_int("D".to_string(),));
    assert_eq!(700, roman_to_int("DCC".to_string()));
    assert_eq!(900, roman_to_int("CM".to_string()));
    assert_eq!(1000, roman_to_int("M".to_string(),));
    assert_eq!(1994, roman_to_int("MCMXCIV".to_string(),));
}
