//! Реализация atoi.
//! Преобразование строки в число.
//! Если переполнение для i32, то вернуть максимум или минимум.
//! Если нет символов, то вернуть 0.
//! Принцып парсинга строки.
//! - Пропустить все пробелы.
//! - Допускается знак числа.
//! - Брать символы числа, если не число, то прекратить парсинг.
//!
//! Ссылка на задачу: https://leetcode.com/problems/string-to-integer-atoi/
//!
#![cfg(test)]

pub fn my_atoi(s: String) -> i32 {
    let mut result: i32 = 0;
    let mut neg = false;
    let mut overwlow = false;

    for (p, ch) in s
        .chars()
        .skip_while(|c| c.is_whitespace())
        .take_while(|v| matches!(v, '-' | '+') || v.is_numeric())
        .enumerate()
    {
        let num = match ch {
            '-' | '+' if p != 0 => {
                break;
            }
            '-' => {
                neg = true;
                continue;
            }
            '+' => {
                continue;
            }
            _ => ch.to_digit(10).unwrap() as i32,
        };
        result = if let Some(v) = result.checked_mul(10).and_then(|v| v.checked_add(num)) {
            v
        } else {
            overwlow = true;
            break;
        };
    }

    if neg {
        if let Some(v) = result.checked_neg() {
            result = v;
        } else {
            overwlow = true;
        }
    }

    if overwlow {
        if neg { i32::MIN } else { i32::MAX }
    } else {
        result
    }
}

#[test]
fn test_my_atoi() {
    assert_eq!(my_atoi("42".to_string()), 42);
    assert_eq!(my_atoi("-42".to_string()), -42);
    assert_eq!(my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(my_atoi("words and 987".to_string()), 0);
    assert_eq!(my_atoi("-91283472332".to_string()), -2147483648);
    assert_eq!(my_atoi("91283472332".to_string()), 2147483647);
    assert_eq!(my_atoi("+1".to_string()), 1);
    assert_eq!(my_atoi("   -42".to_string()), -42);
    assert_eq!(my_atoi("2147483648".to_string()), 2147483647);
    assert_eq!(my_atoi("-2147483649".to_string()), -2147483648);
    assert_eq!(my_atoi("-2147483648".to_string()), -2147483648);
    assert_eq!(my_atoi("  -0012a42".to_string()), -12);
    assert_eq!(my_atoi("-0".to_string()), 0);
    assert_eq!(my_atoi("-".to_string()), 0);
    assert_eq!(my_atoi("+".to_string()), 0);
    assert_eq!(my_atoi("".to_string()), 0);
    assert_eq!(my_atoi(" ".to_string()), 0);
    assert_eq!(my_atoi(" -042".to_string()), -42);
    assert_eq!(my_atoi("-5-".to_string()), -5);
}
