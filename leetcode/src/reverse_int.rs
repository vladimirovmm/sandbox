//! Вернуть целое число в обратном порядке.
//! Если целое число после переворота станет слишком большим или слишком маленьким, вернуть 0.

#![cfg(test)]

pub fn reverse(x: i32) -> i32 {
    let mut remains = x;
    let mut result: i32 = 0;

    loop {
        if remains == 0 {
            break;
        }
        let Some(v) = result.checked_mul(10) else {
            return 0;
        };
        result = v + remains % 10;

        remains /= 10;
        if remains == 0 {
            break;
        }
    }

    result
}

#[test]
fn test_reverse() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-123), -321);
    assert_eq!(reverse(120), 21);
    assert_eq!(reverse(0), 0);
    assert_eq!(reverse(1534236469), 0);
    assert_eq!(reverse(-1534236469), 0);
}
