const ROMAN_NUMERALS: &[(i32, &str)] = &[
    (1, "I"),
    (5, "V"),
    (10, "X"),
    (50, "L"),
    (100, "C"),
    (500, "D"),
    (1000, "M"),
];

pub struct Solution;

impl Solution {
    fn to_roman(num: i32) -> String {
        for (arab, roman) in ROMAN_NUMERALS {
            if num == *arab {
                return roman.to_string();
            }
        }
        String::new()
    }

    /// Преобразует целое число в римское число.
    ///
    /// Ограничения:
    /// 1 <= num <= 3999
    pub fn int_to_roman(num: i32) -> String {
        if num == 0 {
            return Default::default();
        }
        if !(1..=3999).contains(&num) {
            // По условию задачи num <= 3999 и num >= 1
            unimplemented!();
        }

        let mut v = 1;
        while v * 10 <= num {
            v *= 10;
        }

        if v * 9 <= num {
            Self::to_roman(v) + &Self::to_roman(v * 10) + &Self::int_to_roman(num - v * 9)
        } else if v * 5 <= num {
            Self::to_roman(v * 5) + &Self::int_to_roman(num - v * 5)
        } else if v * 4 <= num {
            Self::to_roman(v) + &Self::to_roman(v * 5) + &Self::int_to_roman(num - v * 4)
        } else {
            Self::to_roman(v) + &Self::int_to_roman(num - v)
        }
    }
}

/// Пример 1:
///
/// Входные данные: num = 3749
/// Выходные данные: "MMMDCCXLIX"
///
/// Объяснение:
///
/// 3000 = MMM как 1000 (M) + 1000 (M) + 1000 (M)
///  700 = DCC как 500 (D) + 100 (C) + 100 (C)
///   40 = XL как 10 (X) меньше 50 (L)
///    9 = IX как 1 (I) меньше 10 (X)
/// Примечание: 49 не равно 1 (I) меньше 50 (L), потому что преобразование основано на десятичных разрядах
///
/// Пример 2:
///
/// Входные данные: num = 58
/// Выходные данные: "LVIII"
///
/// Объяснение:
///
/// 50 = L
///  8 = VIII
///
/// Пример 3:
///
/// Входные данные: num = 1994
/// Выходные данные: "MCMXCIV"
///
/// Объяснение:
///
/// 1000 = M
///  900 = CM
///   90 = XC
///    4 = IV
///
/// Ограничения:
/// 1 <= num <= 3999
///
#[test]
fn test_int_to_roman() {
    assert_eq!(Solution::int_to_roman(1), "I");
    assert_eq!(Solution::int_to_roman(2), "II");
    assert_eq!(Solution::int_to_roman(3), "III");
    assert_eq!(Solution::int_to_roman(4), "IV");
    assert_eq!(Solution::int_to_roman(5), "V");
    assert_eq!(Solution::int_to_roman(6), "VI");
    assert_eq!(Solution::int_to_roman(7), "VII");
    assert_eq!(Solution::int_to_roman(8), "VIII");
    assert_eq!(Solution::int_to_roman(9), "IX");
    assert_eq!(Solution::int_to_roman(10), "X");

    assert_eq!(Solution::int_to_roman(3999), "MMMCMXCIX");
    assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX");
    assert_eq!(Solution::int_to_roman(58), "LVIII");
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    assert_eq!(Solution::int_to_roman(1000), "M");
    assert_eq!(Solution::int_to_roman(900), "CM");
    assert_eq!(Solution::int_to_roman(90), "XC");
    assert_eq!(Solution::int_to_roman(4), "IV");
}
