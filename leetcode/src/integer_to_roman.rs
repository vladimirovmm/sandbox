//! Семь различных символов представляют собой римские цифры со следующими значениями:
//!
//! Значение символа
//!     I       1
//!     V       5
//!     X       10
//!     L       50
//!     C       100
//!     D       500
//!     M       1000
//! Римские цифры формируются путем преобразования значений десятичных разрядов от старшего к младшему. Преобразование десятичного разряда в римскую цифру выполняется по следующим правилам:
//!
//! Если значение не начинается с 4 или 9, выберите символ максимального значения, которое можно вычесть из входных данных, добавьте этот символ к результату, вычтите его значение и преобразуйте остаток в римскую цифру.
//! Если значение начинается с 4 или 9, используйте вычитающую форму, представляющую один символ, вычитаемый из следующего символа, например, 4 - это 1 (I) меньше, чем 5 (V): IV, а 9 - это 1 (I) меньше, чем 10 (X): IX. Только следующие вычитающие формы могут использоваться используемые размеры: 4 (IV), 9 (IX), 40 (XL), 90 (XC), 400 (CD) и 900 (CM).
//! Только степени 10 (I, X, C, M) могут быть добавлены последовательно не более 3 раз, чтобы представлять числа, кратные 10. Вы не можете добавлять 5 (V), 50 (L) или 500 (D) несколько раз. Если вам нужно добавить символ 4 раза, используйте вычитающую форму.
//! Получив целое число, преобразуйте его в римскую цифру.
//!
//!  
//!
//! Пример 1:
//!
//! Ввод: число = 3749
//!
//! Вывод: "MMMDCCXLIX"
//!
//! Объяснение:
//!
//! 3000 = МММ как 1000 (М) + 1000 (М) + 1000 (М)
//!  700 = DCC как 500 (D) + 100 (C) + 100 (C)
//!   40 = XL как 10 (X) меньше 50 (L)
//!    9 = IX как 1 (I) меньше 10 (X)
//! Примечание: 49 не на 1 (I) меньше 50 (L), поскольку преобразование основано на десятичных разрядах
//! Пример 2:
//!
//! Ввод: число = 58
//!
//! Вывод: "LVIII"
//!
//! Объяснение:
//!
//! 50 = L
//!  8 = VIII
//! Пример 3:
//!
//! Ввод: число = 1994
//!
//! Вывод: "MCMXCIV"
//!
//! Объяснение:
//!
//! 1000 = М
//!  900 = СМ
//!   90 = XC
//!    4 = IV
//!  
//!
//! Ограничения:
//!
//! 1 <= число <= 3999

#![cfg(test)]

use crate::roman_to_int::roman_to_int;

// Красивее 1ms
pub fn int_to_roman(num: i32) -> String {
    const ENC: [(i32, &str); 13] = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut cursor = ENC.iter();
    let mut cursor_value = cursor.next().unwrap();

    let mut num = num;
    let mut result = String::new();
    while num > 0 {
        if cursor_value.0 > num {
            cursor_value = cursor.next().unwrap();
            continue;
        }
        num -= cursor_value.0;
        result.push_str(cursor_value.1);
    }

    result
}

#[test]
fn test_int_to_roman() {
    for num in 0..4000 {
        let roman = int_to_roman(num);
        assert_eq!(num, roman_to_int(roman))
    }
}

// быстрее 0ms
pub fn int_to_roman_2(num: i32) -> String {
    let mut num = num;
    let mut result = String::new();
    while num > 0 {
        // 90-1000
        if num >= 90 {
            // 400-1000
            if num >= 400 {
                // 900-1000
                if num >= 900 {
                    // 1000
                    if num >= 1000 {
                        result.push('M');
                        num -= 1000;
                    }
                    // 900
                    else {
                        result.push_str("CM");
                        num -= 900;
                    }
                }
                // 400-900
                else if num >= 500 {
                    result.push('D');
                    num -= 500;
                } else {
                    result.push_str("CD");
                    num -= 400;
                }
            }
            // 90-400
            else if num >= 100 {
                result.push('C');
                num -= 100;
            } else {
                result.push_str("XC");
                num -= 90;
            }
        }
        // 9--90
        else {
            // 9-89
            if num >= 9 {
                // 40-89
                if num >= 40 {
                    if num >= 50 {
                        result.push('L');
                        num -= 50;
                    } else {
                        result.push_str("XL");
                        num -= 40;
                    }
                }
                // 9-39
                else if num >= 10 {
                    result.push('X');
                    num -= 10;
                } else {
                    result.push_str("IX");
                    num -= 9;
                }
            }
            // 1-8
            else {
                if num >= 4 {
                    if num >= 5 {
                        result.push('V');
                        num -= 5;
                        continue;
                    } else {
                        result.push_str("IV");
                    }
                } else {
                    result.push_str(&"I".repeat(num as usize));
                }
                num = 0;
            }
        }
    }

    result
}

#[test]
fn test_int_to_roman_2() {
    for num in 0..4000 {
        let roman = int_to_roman_2(num);
        assert_eq!(num, roman_to_int(roman))
    }
}
